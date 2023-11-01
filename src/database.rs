use std::fs::File;
use std::io::{Read, Write};
use std::ops::IndexMut;
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
pub struct GeneVariations(Vec<Option<(AminoAcid, AminoAcidMap<AmClass>)>>);

impl GeneVariations {
    pub fn pathogenicity(&self, index: u16, variation: AminoAcid) -> Option<AmClass> {
        let index = index as usize;
        self.0.get(index).and_then(|e| e.as_ref().map(|(_, map)| map[variation]))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBase(HashMap<String, GeneVariations>);

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
        //Generamos y asignamos la memoria para la estructura final
        let mut genes: HashMap<String, GeneVariations> = rows()
            .enumerate()
            .fold(
                HashMap::new(),
                |mut map,
                 (
                    n,
                    Row {
                        uniprot_id,
                        protein_variant,
                        am_class: _,
                    },
                )| {
                    percentage_completed("Conteo del tamaño de los genes", n, 216175355);
                    if map.contains_key(&uniprot_id) {
                        let v = *map
                            .get(&uniprot_id)
                            .expect("El condicional de arriba asegura que si existe la llave");
                        if protein_variant.position > v {
                            map.insert(uniprot_id.to_owned(), protein_variant.position);
                        }
                    } else {
                        map.insert(uniprot_id.to_owned(), protein_variant.position);
                    }

                    map
                },
            )
            .into_iter()
            .enumerate()
            .map(|(n, (k, v))| {
                percentage_completed("Creación de la estructura", n, 20516);
                let mut vec = Vec::new();
                vec.resize_with(v.into(), || None);
                (k, GeneVariations(vec))
            })
            .collect();

        for (
            n,
            Row {
                uniprot_id,
                protein_variant,
                am_class,
            },
        ) in rows().enumerate()
        {
            percentage_completed("Añadiendo Datos", n, 216175355);
            let Variation {
                base,
                position,
                variant,
            } = protein_variant;

            let index = position as usize - 1;

            let slot = genes
                .get_mut(&uniprot_id)
                .expect("Todas las IDs están contenidas en un HashMap")
                .0
                .index_mut(index);

            match slot {
                Some((_, map)) => map[variant] = am_class,
                None => {
                    *slot = {
                        let mut map = AminoAcidMap([(); 20].map(|_| AmClass::Undefined));
                        map[variant] = am_class;
                        Some((base, map))
                    }
                }
            }
        }

        DataBase(genes)
    }

    pub fn serialize(&self) {
        let options = bincode::config::DefaultOptions::new().with_varint_encoding();

        let serialized = options.serialize(&self).expect("Serialización fallida");
        let mut file = File::create("variations.cdv").expect("Error al crear el archivo");

        file.write_all(&serialized)
            .expect("No se pudo guardar la información");
    }

    pub fn _genes_as_json(&self) {
        let keys: Vec<_> = self.0.keys().collect();
        let serialized = serde_json::to_string(&keys).unwrap();

        println!("{}", serialized)
    }

    pub fn _serialize_to_json(&self) {
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
        &self.0[key]
    }
}

/// Regresa un iterador sobre las filas de Alphamissense
fn rows() -> Box<dyn Iterator<Item = Row>> {
    let path = "AlphaMissense_aa_substitutions.tsv";
    let file =
        File::open(path).expect("El programa necesita las predicciones de AlphaMissense");
    Box::new(std::io::BufReader::new(file)
    .lines()
    .skip(4)
    .map(|row| Row::from_str(
        &row.expect("El buffer debe de poder leer.")
    ).expect("El formato del documento debe de poder parsearse.")))
}