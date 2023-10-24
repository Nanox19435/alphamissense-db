use std::{collections::HashMap, io::BufRead};
use std::fs::File;
use std::str::FromStr;
use std::io::{Write, Read};

use bincode::Options;
use serde::{Serialize, Deserialize};

use crate::{
    aminoacids::{AminoAcidMap, AminoAcid},
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
            _ =>  return Err(()),
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
struct GeneVariations{
    base: AminoAcid, 
    variants: Vec<AminoAcidMap<AmClass>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBase {
    map: HashMap<String, GeneVariations>,
}

fn percentage_completed(procedure: &str, count: usize, total: usize) {
    let c = 100 * count;
    let p = total/100;

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
        file.read_to_end(&mut buf).expect("Error al leer la base de datos");

        let options = bincode::config::DefaultOptions::new()
        .with_varint_encoding();

        options.deserialize(&buf).expect("Error en la deserialización de la Base de Datos")
    }

    pub fn new() -> Self {
        type M = HashMap<String, Vec<(Variation, AmClass)>>;

        let path = "AlphaMissense_aa_substitutions.tsv";
        let file = File::open(path).expect("Error al leer el archivo");
        let buffer = std::io::BufReader::new(file);

        // Iteramos sobre las lineas, primero para crear un iterador de structs Row, es decir, parseamos.
        let map: HashMap<String, GeneVariations> = buffer
            .lines()
            .enumerate()
            .skip(4)
            .map(|(n, row)| {
                //percentage_completed("Procesamiento de las filas", n, 216175355);
                let row = row.expect(&format!("Error de lectura en línea: {}", n));
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
            // Una vez tenemos relacionados todos los genes con todas sus variaciones, ordenamos las variaciones
            // y juntamos las variaciones sobre la misma base en un único AminoAcidMap
            .map(|(uniprot_id, mut v)| {
                // Ordenamos usando la posición 
                v.sort_by(|(a, _), (b, _)| a.position.cmp(&b.position));
                let mut b = AminoAcid::Alanine;
                let variants = v.into_iter()
                    .fold(Vec::new(), |mut variants, (variation, class)| {
                        let Variation { base, position, variant } = variation;
                        // si la posición se encuentra fuera del vec, lo extendemos
                        b = base;
                        if variants.len() < position as usize {
                            let mut map = AminoAcidMap([();20].map(|_| None));
                            map[variant] = Some(class);
                            variants.push(map)
                        } else {
                            unsafe {
                                variants.last_mut().unwrap_unchecked()[variant] = Some(class)
                            }
                        }
                        variants
                    })
                    .into_iter()
                    .map(|AminoAcidMap(map)| {
                        AminoAcidMap(map.map(|e| {
                            match e {
                                Some(class) => class,
                                None => AmClass::Undefined,
                            }
                        }))
                    }).collect();
                    let gene_variations = GeneVariations {
                        base: b,
                        variants
                    };
                (uniprot_id, gene_variations)
            })
            .collect();
        DataBase { map }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn serialize(&self) {
        let options = bincode::config::DefaultOptions::new()
        .with_varint_encoding();

        let serialized = options.serialize(&self).expect("Serialización fallida");
        let mut file = File::create("variations.cdv").expect("Error al crear el archivo");

        file.write_all(&serialized).expect("No se pudo guardar la información");
    }

    pub fn _genes_as_json(&self) {
        let keys: Vec<_> = self.map.keys().collect();
        let serialized = serde_json::to_string(&keys).unwrap();

        println!("{}", serialized)
    }
}


pub fn serialize_names(names: &HashMap<String, String>) {
    let options = bincode::config::DefaultOptions::new()
        .with_varint_encoding();
    let serialized = options.serialize(names).expect("Error serializando los nombres");
    
    let mut file = File::create("gene_names.cdv").expect("Error al crear el archivo");

    file.write_all(&serialized).expect("No se pudo guardar la información");
}