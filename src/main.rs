use rocket::State;

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

#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn query(name: &str, index: &State<tantivy::Index>) -> String {
    let resultados = search::search(index, name)
        .expect("Si hay un error con Tantivy, no podemos continuar con el programa.");
    serde_json::to_string(&resultados)
        .unwrap_or("Error en la búsqueda".to_owned())
}

//#[get("/<name>/<variant>")]

#[launch]
fn rocket() -> _ {
    let database = database::DataBase::open();
    let index = search::index();
    rocket::build()
        .manage(database)
        .manage(index)
        .mount("/hello", routes![world])
        .mount("/search", routes![query])
}
