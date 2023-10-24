use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;
use std::{collections::HashMap, io::BufRead};

use bincode::Options;
use serde::{Deserialize, Serialize};

use crate::{
    aminoacids::{AminoAcid, AminoAcidMap},
    variations::{AmClass, Variation},
};

/// Representa la información en una fila de la tabla de sustituciones.
pub struct Row {
    uniprot_id: String,
    protein_variant: Variation,
    am_class: AmClass,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(row: &str) -> Result<Self, Self::Err> {
        let mut values = row.split("\t");

        let uniprot_id = values.next().ok_or(())?.to_owned();
        let protein_variant = Variation::from_str(values.next().ok_or(())?)?;
        let pathogenicity = values.next().ok_or(())?.parse().map_err(|_| ())?;
        let am_class = match values.next().ok_or(())? {
            "benign" => AmClass::Benign(pathogenicity),
            "pathogenic" => AmClass::Pathogenic(pathogenicity),
            "ambiguous" => AmClass::Ambiguous(pathogenicity),
            _ => return Err(()),
        };

        Ok(Row {
            uniprot_id,
            protein_variant,
            am_class,
        })
    }
}

/// función posición en el gen: usize -> (Ali -> AmClass)
#[derive(Serialize, Deserialize, Debug)]
pub struct GeneVariations {
    variants: Vec<(AminoAcid, AminoAcidMap<AmClass>)>,
}

impl GeneVariations {
    pub fn pathogenicity(&self, index: u16, variation: AminoAcid) -> Option<AmClass> {
        let index = index as usize;
        self.variants.get(index).map(|(_, map)| map[variation])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBase {
    map: HashMap<String, GeneVariations>,
}

fn percentage_completed(procedure: &str, count: usize, total: usize) {
    let c = 100 * count;
    let p = total / 100;

    if count % p == 0 {
        println!("{}: {}%", procedure, c / total);
    }
}

impl DataBase {
    const PATH: &'static str = "variations.cdv";

    pub fn open() -> Self {
        let path = std::path::Path::new(DataBase::PATH);

        if path.exists() {
            DataBase::load()
        } else {
            let database = DataBase::new();
            database.serialize();

            database
        }
    }

    pub fn load() -> Self {
        let mut file = File::open(DataBase::PATH).expect("Error al abrir la base de datos");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .expect("Error al leer la base de datos");

        let options = bincode::config::DefaultOptions::new().with_varint_encoding();

        options
            .deserialize(&buf)
            .expect("Error en la deserialización de la Base de Datos")
    }

    pub fn new() -> Self {
        type M = HashMap<String, Vec<(Variation, AmClass)>>;

        let path = "AlphaMissense_aa_substitutions.tsv";
        let file =
            File::open(path).expect("El programa necesita las predicciones de AlphaMissense");
        let buffer = std::io::BufReader::new(file);

        // Iteramos sobre las lineas, primero para crear un iterador de structs Row, es decir, parseamos.
        let map: HashMap<String, GeneVariations> = buffer
            .lines()
            .enumerate()
            .skip(4)
            .map(|(n, row)| {
                percentage_completed("Procesamiento de las filas", n, 216175355);
                let row = row.expect("");
                Row::from_str(&row).expect(&format!("Fila inválida: {}", n))
            })
            // Ahora, usamos un HashMap para relacionar un gen con todas sus posibles variaciones
            .fold(HashMap::new(), |mut map: M, row| {
                let Row {
                    uniprot_id,
                    protein_variant,
                    am_class,
                } = row;
                if map.contains_key(&uniprot_id) {
                    map.get_mut(&uniprot_id)
                        .unwrap()
                        .push((protein_variant, am_class));
                } else {
                    map.insert(uniprot_id, vec![(protein_variant, am_class)]);
                }

                map
            })
            .into_iter()
            // Una vez tenemos relacionados todos los genes con todas sus variaciones, juntamos en un hashmap las variaciones sobre el mismo codón.
            .map(|(uniprot_id, variations)| {
                type T = BTreeMap<u16, (AminoAcid, AminoAcidMap<AmClass>)>;
                let variations = variations.into_iter().fold(
                    BTreeMap::new(),
                    |mut variations: T, (variation, class)| {
                        let Variation {
                            base,
                            position,
                            variant,
                        } = variation;

                        if let Some((_, map)) = variations.get_mut(&position) {
                            map[variant] = class;
                        } else {
                            variations.insert(
                                position,
                                (base, AminoAcidMap([(); 20].map(|_| AmClass::Undefined))),
                            );
                        }

                        variations
                    },
                ); // En este iterador, los valores están ordenados, y los iremos insertando en órden

                // Almacenamos en un vector por motivos de rendimiento y de memoria
                // También, prealocamos la memoria que creemos que va a requerir. Esto debería de acelerar el procedimiento.
                let (last_position, _) = variations
                    .last_key_value()
                    .expect("El mapa no puede estar vacío");
                let mut variation_string = Vec::with_capacity(*last_position as usize);
                let mut counter = 0;
                for (position, (base, variations)) in variations.into_iter() {
                    if counter < position {
                        // Llenamos con Undefined la diferencia de posiciones
                        variation_string.extend((counter..position)
                            .map(|_| (base, AminoAcidMap([(); 20].map(|_| AmClass::Undefined)))));
                        counter = position;
                    }
                    // Asignamos la clase de patogenicidad a la posición que le corresponde
                    variation_string[position as usize - 1] = (base, variations);
                }

                (
                    uniprot_id,
                    GeneVariations {
                        variants: variation_string,
                    },
                )
            })
            .collect();
        DataBase { map }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn serialize(&self) {
        let options = bincode::config::DefaultOptions::new().with_varint_encoding();

        let serialized = options.serialize(&self).expect("Serialización fallida");
        let mut file = File::create("variations.cdv").expect("Error al crear el archivo");

        file.write_all(&serialized)
            .expect("No se pudo guardar la información");
    }

    pub fn _genes_as_json(&self) {
        let keys: Vec<_> = self.map.keys().collect();
        let serialized = serde_json::to_string(&keys).unwrap();

        println!("{}", serialized)
    }

    pub fn serialize_to_json(&self) {
        // Serialize the data to JSON
        let json_data = serde_json::to_string(&self).unwrap();

        // Create or open the "prueba.json" file for writing
        let mut file = File::create("prueba.json").unwrap();

        // Write the JSON data to the file
        file.write_all(json_data.as_bytes()).unwrap();
    }

    pub fn _deserialize_from_json(&self) -> Self {
        // Open the "prueba.json" file for reading
        let mut file = File::open("prueba.json").unwrap();
        let mut json_data = String::new();

        // Read the contents of the file into a String
        file.read_to_string(&mut json_data).unwrap();

        // Deserialize the JSON data into a Prueba struct
        let data: Self = serde_json::from_str(&json_data).unwrap();

        data
    }

    pub fn get(&self, key: &str) -> &GeneVariations {
        &self.map[key]
    }
}
