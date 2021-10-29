use crate::defaults::*;

pub mod defaults {
    pub use crate::utils::*;
    pub use eframe::{ egui, epi };

    pub use cello_model::{ View, Table };
}


#[macro_use]
mod utils;
mod elements;


#[derive(Default, Clone, Copy)]
struct App;

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let mut table =
        elements::Column::with_children("Hello world!".label(), vec![
            elements::Row::new("foo".label()).as_box_table(),
            elements::Row::new("bar".label()).as_box_table(),
            elements::Row::new("baz".label()).as_box_table(),
        ]);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Foo");
            ui.separator();

            table.view(ui);
        });

        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "cello"
    }
}


fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(App), options)
}
