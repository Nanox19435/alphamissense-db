use std::collections::HashMap;

use crate::{aminoacids::{AminoAcidMap, self}, variations::{AmClass, Variation}, builder::Row};

#[derive(Hash, PartialEq, Eq)]
struct UniprotId(String);

/// función posición en el gen: usize -> (Ali -> AmClass)
struct GeneVariations(Vec<AminoAcidMap<AmClass>>);
pub struct DataBase {
    map: HashMap<UniprotId, GeneVariations>,
}

pub enum DataBaseCreationError {
    FileError,
}

impl DataBase {
    pub fn new() -> Self {
        let data_path = "AlphaMissense_aa_substitutions.tsv";
        let table = super::Table::build_table(data_path);

        type M = HashMap<String, Vec<(Variation, AmClass)>>;
        table.0.iter().fold(HashMap::new(), |mut map: M, row| {
            let Row {
                uniprot_id,
                protein_variant,
                am_class,
            } = row;
            if map.contains_key(&uniprot_id) {
                let  variations = map.get_mut(uniprot_id).unwrap().push((protein_variant, am_class));
            } else {
                map.insert(uniprot_id, vec![(protein_variant, am_class)]);
            }

            map
        });
        todo!()
    }
}