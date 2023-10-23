use std::collections::HashMap;

use crate::{aminoacids::{AminoAcidMap, self}, variations::{AmClass, Variation}, builder::Row};

/// función posición en el gen: usize -> (Ali -> AmClass)
struct GeneVariations(Vec<AminoAcidMap<AmClass>>);
pub struct DataBase {
    map: HashMap<String, GeneVariations>,
}

pub enum DataBaseCreationError {
    FileError,
}

impl DataBase {
    pub fn new() -> Self {
        let data_path = "AlphaMissense_aa_substitutions.tsv";
        let table = super::Table::build_table(data_path);

        type M = HashMap<String, Vec<(Variation, AmClass)>>;
        let map = table.0.into_iter().fold(HashMap::new(), |mut map: M, row| {
            let Row {
                uniprot_id,
                protein_variant,
                am_class,
            } = row;
            if map.contains_key(&uniprot_id) {
                map.get_mut(&uniprot_id).unwrap().push((protein_variant, am_class));
            } else {
                map.insert(uniprot_id, vec![(protein_variant, am_class)]);
            }

            map
        }).into_iter().map(|(k, mut v)| {
            v.sort_by(|(a, _), (b, _)| {
                a.position.cmp(&b.position)
            });


        }).collect();


        
        todo!()
    }
}


