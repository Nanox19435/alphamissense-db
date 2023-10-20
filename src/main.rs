/// Módulo donde se definen las operaciones para construir la Base de Datos
mod builder;

mod uniprot;
/// Módulo donde se definen operaciones refentes a Aminoacidos.
mod aminoacids;
/// Módulo donde se definen operaciones sobre variaciones.
mod variations;

fn main() {
    //uniprot::get_genes_names("AlphaMissense_aa_substitutions.tsv");
    let table = builder::Table::build_table("AlphaMissense_aa_substitutions.tsv");
}
