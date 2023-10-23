/// Módulo donde se definen las operaciones para construir la Base de Datos
mod database;
/// Módulo donde se define la operación para extraer el nombre de un gen, dada su id en uniprot.
mod uniprot;
/// Módulo donde se definen operaciones refentes a Aminoacidos.
mod aminoacids;
/// Módulo donde se definen operaciones sobre variaciones.
mod variations;

fn main() {
    let database = database::DataBase::open();
    //println!("Base de datos cargada. Longitud: {}", database.len());
    database._as_json();

    //let names = database.get_names();

    //database::serialize_names(&names);
}
