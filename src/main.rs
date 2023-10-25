use std::str::FromStr;

use rocket::{State, response::status};
use variations::Variation;

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

#[get("/<id>/<variant>")]
fn get_variants(id: &str, variant: &str, database: &State<database::DataBase>) -> status::Accepted<String> {
    status::Accepted(
        Variation::from_str(variant).ok()
        .and_then(|Variation { base, position, variant }| {
            if base != variant {
                database.get(id)
                .pathogenicity(position, variant)
                .map(|i| i.to_string())
            } else {
                Some("Silenciosa".to_owned())
            }
        })
    )
}

#[launch]
fn rocket() -> _ {
    let database = database::DataBase::open();
    let index = search::index().expect("El programa necesita que se inicie Tanitvy");
    rocket::build()
        .manage(database)
        .manage(index)
        .mount("/hello", routes![world])
        .mount("/search", routes![query])
        .mount("/variants", routes![get_variants])
}
