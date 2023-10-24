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
/// Módulo donde se define la Interzad de Usuario
mod ui;

fn main() -> tantivy::Result<()> {
    let database = database::DataBase::open();
    println!("Base de datos cargada. Longitud: {}", database.len());

    let index = search::index()?;

    ui::main_ui(database, index).expect("Interfaz de usuario debe de poder construirse");

    Ok(())
}
