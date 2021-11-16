use crate::defaults::*;

use super::Ui;


#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nothing;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Label<S>(pub S);


impl View<Ui<'_>, ()> for Nothing {
    fn view(&mut self, _: super::Ui) {}
}
impl Table<Ui<'_>, ()> for Nothing {
    type Title = Self;
    type Child = super::BoxTable<Self>;

    fn title(&self) -> &Self::Title {
        self
    }
    fn title_mut(&mut self) -> &mut Self::Title {
        self
    }

    fn content(&self) -> &[Self::Child] {
        &[]
    }

    fn content_mut(&mut self) -> &mut [Self::Child] {
        &mut []
    }

    fn len(&self) -> usize {
        0
    }
}


impl<S: AsRef<str>> View<Ui<'_>, ()> for Label<S> {
    fn view(&mut self, ui: super::Ui) {
        ui.label(self.0.as_ref());
    }
}
impl<S: AsRef<str>> Table<Ui<'_>, ()> for Label<S> {
    type Title = Self;
    type Child = super::BoxTable<Self>;

    fn title(&self) -> &Self::Title {
        self
    }
    fn title_mut(&mut self) -> &mut Self::Title {
        self
    }

    fn content(&self) -> &[Self::Child] {
        &[]
    }

    fn content_mut(&mut self) -> &mut [Self::Child] {
        &mut []
    }

    fn len(&self) -> usize {
        0
    }
}


#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Selectable<V> {
    selected: bool,
    inner: V,
}


impl<V> From<V> for Selectable<V> {
    fn from(inner: V) -> Selectable<V> {
        Self {
            selected: false,
            inner,
        }
    }
}

impl<V: for<'ui> View<Ui<'ui>, ()>> View<Ui<'_>, ()> for Selectable<V> {
    fn view(&mut self, ui: super::Ui) {
        let mut frame = egui::Frame::default();

        if self.selected {
            frame.fill = egui::Color32::LIGHT_GRAY;
        }

        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.selected, "");

                self.inner.view(ui);
            })
        });
    }
}

impl<'s, V: for<'ui> Table<Ui<'ui>, ()>> Table<Ui<'s>, ()> for Selectable<V> {
    type Title = <V as Table<Ui<'s>, ()>>::Title;

    type Child = <V as Table<Ui<'s>, ()>>::Child;

    fn title(&self) -> &Self::Title {
        self.inner.title()
    }
    fn title_mut(&mut self) -> &mut Self::Title {
        self.inner.title_mut()
    }

    fn content(&self) -> &[Self::Child] {
        self.inner.content()
    }

    fn content_mut(&mut self) -> &mut [Self::Child] {
        self.inner.content_mut()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<V: for<'ui> TableMut<Ui<'ui>, ()>> TableMut<Ui<'_>, ()> for Selectable<V> {
    fn insert(&mut self, at: usize, cell: Self::Child) {
        self.inner.insert(at, cell)
    }
}


impl<T> Checkable for Selectable<T> {
    fn check(&self) -> bool {
        self.selected
    }
}
