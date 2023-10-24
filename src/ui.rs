use std::str::FromStr;

use tantivy::Index;

use crate::{database::DataBase, search::search, variations::{Variation, AmClass}};

/// Representación del estado interno del programa
enum State {
    /// Buscando el gen
    Searching(String, Vec<(String, String)>),
    /// Seleccionando la mutación
    Selecting{
        gene_name: String,
        id: String,
        input: String,
        result: Option<AmClass>
    }
}

/// Función principal de Interfaz de Usuario. En el futuro, planeo reemplazar la biblioteca de interfaz de usuario porque egui deja mucho que desear y es muy poco mantenible.
pub fn main_ui(database: DataBase, index: Index) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };

    eframe::run_native(
        "Alphamissense Conde de Valenciana", 
        native_options, 
        Box::new(|_| Box::new(App::new(database, index))),
    )
}

struct App {
    database: DataBase,
    index: Index,
    state: State,
}

impl App {
    fn new(database: DataBase, index: Index) -> Self {
        App {
            database,
            index,
            state: State::Searching(
                String::new(), 
                Vec::new()
            ),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let transition: Option<State> = match &mut self.state {
            State::Searching(input, query) => {
                let mut transition = None;
                egui::TopBottomPanel::top("Herramientas").show(ctx, |ui| {
                    ui.label("Aquí iría un menú de herramientas");
                });
                
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Introduzca el nombre del gen: ");
        
                        if ui.text_edit_singleline(input).changed() {
                            *query = search(&self.index, input).unwrap_or_default();
                        }
                    });
        
                    egui::Grid::new("Resultados de búsqueda").show(ui, |ui| {
                        for (uniprot_id, gene) in query {
                            if ui.button(gene.as_str()).clicked() {
                                transition = Some(
                                    State::Selecting { 
                                        gene_name: gene.to_owned(), 
                                        id: uniprot_id.to_owned(), 
                                        input: String::new(),
                                        result: None
                                    });
                            }
                            ui.end_row();
                        }
                    });
                });

                transition
            },
            State::Selecting{ gene_name, id, input, result} => {
                let mut transition = None;
                egui::TopBottomPanel::top("Herramientas").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(gene_name.as_str());
                        if ui.button("ATRÁS").clicked() {
                            transition = Some(State::Searching(
                                String::new(),
                                Vec::new()
                            ));
                        }
                    })
                });

                
                //Esto es horrible y no debería hacerlo, pero este código debería ser reemplazado por una mejor biblioteca de UI.
                egui::CentralPanel::default().show(ctx, |ui| {
                    if ui.text_edit_singleline(input).changed() {
                        let variations = self.database.get(id);
                        *result = if let Ok(
                            Variation { base: _, position, variant }
                        ) = Variation::from_str(&input) { 
                            // TODO mandar mensaje cuando la base del aminoacido difiera a la introducida
                            variations.get(position + 1, variant)
                        } else {
                            //Cambiaríamos el color del texto aquí, si supiera como
                            None
                        };
                    }
                     
                    ui.vertical_centered(|ui| {
                        ui.label(match result {
                            Some(class) => match class {
                                crate::variations::AmClass::Benign(v) => format!("Benigno: {}", v),
                                crate::variations::AmClass::Pathogenic(v) => format!("Patogénico: {}", v),
                                crate::variations::AmClass::Ambiguous(v) => format!("Ambiguo: {}", v),
                                crate::variations::AmClass::Undefined => "No definido".to_owned(),
                            },
                            None => "Formato de variación inválido".to_owned(),
                        });
                    })
                });

                transition
            },
        };

        if let Some(s) = transition {
            self.state = s;
        }
    }

}