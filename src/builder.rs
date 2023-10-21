use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

use crate::variations::{AmClass, Variation};

/// Representa la información en una fila de la tabla de sustituciones.
pub struct Row {
    uniprot_id: String,
    protein_variant: Variation,
    am_class: AmClass,
}

pub struct Table(Vec<Row>);

impl Table {
    /// Construye la tabla y el conjunto de id's
    pub fn build_table(path: &str) -> Self {
        let file = File::open(path).expect("Error al leer el archivo");
        let buffer = std::io::BufReader::new(file);
        Table(buffer
            .lines()
            .skip(4)
            .enumerate()
            .map(|(n, row)| {
                let row = row.expect(&format!("Error de lectura en línea: {}", n));
                println!("{}", row);
                let mut values = row.split("\t");

                let uniprot_id = values
                    .next()
                    .expect(&format!("Fila inválida en línea: {}", n))
                    .to_owned();
                let protein_variant = Variation::from_str(
                    values
                        .next()
                        .expect(&format!("Fila inválida en línea: {}", n)),
                )
                .expect(&format!("Variación no válida en línea {}", n));
                let pathogenicity = values
                    .next()
                    .expect(&format!("Fila inválida en línea: {}", n))
                    .parse()
                    .expect(&format!("Patogenicidad inválida en fila: {}", n));
                let am_class = match values
                    .next()
                    .expect(&format!("Fila inválida en línea: {}", n))
                {
                    "benign" => AmClass::Benign(pathogenicity),
                    "pathogenic" => AmClass::Pathogenic(pathogenicity),
                    "ambiguous" => AmClass::Ambiguous(pathogenicity),
                    _ => panic!("Clasificación inválida en línea: {}", n),
                };

                Row {
                    uniprot_id,
                    protein_variant,
                    am_class,
                }
            })
            .collect())
    } 

    /// Obtiene los nombres de las proteinas y los guarda en un diccionario.
    fn get_genes(&self) -> HashMap<String, String> {
        let Table(rows) = self;
        rows.iter().map(|row| {
            let id = row.uniprot_id.to_owned();
            let name = crate::uniprot::get_gene_name(&id);

            (id, name)
        }).collect()
    }
}

/// Módulo donde se define la principal estructura contenedora de los datos.
pub mod database;