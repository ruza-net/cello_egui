use crate::defaults::*;

pub mod primitives;

pub use primitives::{ Label, Nothing };


pub type Ui<'ui> = &'ui mut egui::Ui;

type DynTableMut<T> = Box<dyn for<'ui> TableMut<Ui<'ui>, (), Title = T, Child = BoxTable<T>>>;

pub struct BoxTable<T>(DynTableMut<T>);

impl<T> From<DynTableMut<T>> for BoxTable<T> {
    fn from(t: DynTableMut<T>) -> Self {
        Self(t)
    }
}

impl<T> View<Ui<'_>, ()> for BoxTable<T> {
    fn view(&mut self, ui: Ui<'_>) -> () {
        self.0.view(ui)
    }
}
impl<T> Table<Ui<'_>, ()> for BoxTable<T> {
    type Title = T;
    type Child = Self;

    fn title(&self) -> &Self::Title {
        self.0.title()
    }

    fn content(&self) -> &[Self::Child] {
        self.0.content()
    }

    fn content_mut(&mut self) -> &mut [Self::Child] {
        self.0.content_mut()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<T> TableMut<Ui<'_>, ()> for BoxTable<T> {
    fn insert(&mut self, at: usize, cell: Self) {
        self.0.insert(at, cell)
    }
}


macro_rules! vectors {
( $($name:ident as $mode:ident),* $(,)? ) => {
$(

#[derive(Default)]
pub struct $name<T> {
    title: T,
    content: Vec<BoxTable<T>>,
}

impl<T> $name<T> {
    pub fn new(title: T) -> Self {
        Self {
            title,
            content: vec![],
        }
    }

    pub fn with_children(title: T, content: Vec<BoxTable<T>>) -> Self {
        Self { title, content }
    }
}

impl<T: for<'title> View<Ui<'title>, ()>> View<Ui<'_>, ()> for $name<T> {
    fn view(&mut self, ui: Ui) {
        self.title.view(ui);

        if !self.content.is_empty() {
            ui.group(|ui| {
                ui.$mode(|ui| {
                    for child in &mut self.content {
                        child.view(ui);
                    }
                });
            });
        }
    }
}

impl<T: for<'title> View<Ui<'title>, ()>> Table<Ui<'_>, ()> for $name<T> {
    type Title = T;
    type Child = BoxTable<T>;

    fn title(&self) -> &Self::Title {
        &self.title
    }

    fn content(&self) -> &[Self::Child] {
        &self.content
    }
    fn content_mut(&mut self) -> &mut [Self::Child] {
        &mut self.content
    }


    fn len(&self) -> usize {
        self.content.len()
    }
}

impl<T: for<'title> View<Ui<'title>, ()>> TableMut<Ui<'_>, ()> for $name<T> {
    fn insert(&mut self, at: usize, cell: Self::Child) {
        self.content.insert(at, cell)
    }
}

)*
};
}

vectors! { Row as horizontal, Column as vertical }
