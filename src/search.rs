use std::{fs, collections::HashMap};

use tantivy::{collector::TopDocs, directory, query::QueryParser, schema::*, Index, ReloadPolicy};

/// Inicializa el índice de búsqueda. Si no existe, lo crea.
pub fn index() -> tantivy::Result<Index> {
    match fs::create_dir("index") {
        Ok(_) => {
            let dir = directory::MmapDirectory::open("index")?;
            let mut schema_builder = Schema::builder();

            schema_builder.add_text_field("Nombre_Gen", TEXT | STORED);
            schema_builder.add_text_field("Uniprot_ID", TEXT | STORED);
            let schema = schema_builder.build();

            let index = Index::create(dir, schema.clone(), Default::default())?;
            let mut index_writer = index.writer(50_000_000)?;

            let gene = schema.get_field("Nombre_Gen").unwrap();
            let uniprot_id = schema.get_field("Uniprot_ID").unwrap();

            let names: HashMap<String, String> = serde_json::from_str(
                include_str!("genes/names.json")
            ).map_err(|_| {
                tantivy::TantivyError::InternalError(
                    "Failed to initialize the id -> name map.".to_owned()
                )
            })?;

            for (n, (id, name)) in names.into_iter().enumerate() {
                if n % 205 == 0 {
                    println!("{}%", (100*n)/20516);
                }
                index_writer.add_document(doc!(
                    gene => name,
                    uniprot_id => id
                ))?;
            }

            index_writer.commit()?;

            Ok(index)
        }
        Err(_) => {
            let dir = directory::MmapDirectory::open("index")?;
            let index = Index::open(dir)?;

            Ok(index)
        }
    }
}


/// Regresa los 10 genes que mejor coinciden con la búsqueda.
pub fn search(index: &Index, n: &str) -> tantivy::Result<Vec<(String, String)>> {
    let gene = index.schema().get_field("Nombre_Gen")?;
    let uniprot_id = index.schema().get_field("Uniprot_ID")?;
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();

    let mut query_parser = QueryParser::for_index(&index, vec![gene]);
    query_parser.set_field_fuzzy(gene, true, 1, true);
    let query = query_parser.parse_query(n)?;

    let top_results = searcher
        .search(&query, &TopDocs::with_limit(20))?;

    Ok(top_results
        .into_iter()
        .map(|(_, adress)| {
            (searcher
                .doc(adress)
                .expect("El documento fue proveído por el searcher, es decir, lo contiene")
                .get_first(uniprot_id)
                .expect("Todo documento en el index tiene un atributo \"Uniprot_ID\" guardado")
                .as_text()
                .expect("Todo atributo gene es un string")
                .to_string(),
            searcher
                .doc(adress)
                .expect("El documento fue proveído por el searcher, es decir, lo contiene")
                .get_first(gene)
                .expect("Todo documento en el index tiene un atributo \"Nombre_Gen\" guardado")
                .as_text()
                .expect("Todo atributo gene es un string")
                .to_string())
        })
        .collect())
}
