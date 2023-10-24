#[macro_use]
extern crate tantivy;

/// Módulo donde se definen operaciones refentes a Aminoacidos.
mod aminoacids;
/// Módulo donde se definen las operaciones para construir la Base de Datos
mod database;
/// Módulo que define el motor de búsqueda por texto.
mod search;
/// Módulo donde se define la operación para extraer el nombre de un gen, dada su id en uniprot.
mod uniprot;
/// Módulo donde se definen operaciones sobre variaciones.
mod variations;
fn main() -> tantivy::Result<()> {
    //let _database = database::DataBase::open();
    //println!("Base de datos cargada. Longitud: {}", database.len());

    let index = search::index()?;

    for r in search::search(&index, "SPAG")? {
        println!("{}", r)
    }

    Ok(())
}
