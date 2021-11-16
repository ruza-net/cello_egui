use crate::defaults::*;

pub mod defaults {
    pub use crate::utils::*;
    pub use eframe::{ egui, epi };

    pub use cello_model::{ View, Table, TableMut };
}


#[macro_use]
mod utils;
mod elements;

use elements::{ Label, Row, BoxTable, Selectable };


struct App {
    table: BoxTable<Selectable<Label<String>>>,
}
impl Default for App {
    fn default() -> Self {
        Self {
            table: Row::new("Table".to_string().label().into()).to_box_table(),
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Foo");
            ui.separator();

            self.table.view(ui);

            if ui.button("Push").clicked() {
                let push_action =
                |subtable: &mut BoxTable<_>| {
                    subtable.push(
                        Row::new("foo".to_string().label().into()).to_box_table()
                    );
                };

                self.table
                    .walk(push_action);
            }
        });

        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "cello"
    }
}


fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(App::default()), options)
}
