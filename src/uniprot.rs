extern crate simd_json;

use simd_json::ValueAccess;

/// Dada una id de uniprot, consulta a uniprot para conseguir su nombre.
pub fn get_gene_name(uniprot_id: &str) -> String {
    let url = format!("https://www.uniprot.org/uniprot/{}.json", uniprot_id);
    match reqwest::blocking::get(&url) {
        Ok(re) => {
            match re.text() {
                Ok(mut txt) => {
                    let bytes = unsafe {txt.as_bytes_mut()};
                    let parsed_json = simd_json::to_borrowed_value(bytes).expect("Error fatal, no es un json.");
                    let genes = parsed_json.get("genes")
                        .and_then(|g| g.get_idx(0))
                        .and_then(|g| g.get("geneName"))
                        .and_then(|g| g.get("value"))
                        .unwrap_or_else(|| {
                            &parsed_json.get("uniProtkbId").expect("JSon con formato inválido. Es imposible determinar un nombre")
                        });
                    
                    genes.as_str().expect(&url).to_owned()
                },
                Err(e) => todo!(),
            }
        },
        Err(e) => todo!(),
    } 
}