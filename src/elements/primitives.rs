use crate::defaults::*;


#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Nothing;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Label<S>(pub S);

impl View<super::Ui<'_>, ()> for Nothing {
    fn view(&mut self, ui: super::Ui) -> () {}
}
impl Table<super::Ui<'_>, ()> for Nothing {
    type Title = Self;
    type Child = super::BoxTable<Self>;

    fn title(&self) -> &Self::Title {
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


impl<S: AsRef<str>> View<super::Ui<'_>, ()> for Label<S> {
    fn view(&mut self, ui: super::Ui) -> () {
        ui.label(self.0.as_ref());
    }
}
impl<S: AsRef<str>> Table<super::Ui<'_>, ()> for Label<S> {
    type Title = Self;
    type Child = super::BoxTable<Self>;

    fn title(&self) -> &Self::Title {
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

impl<V: for<'ui> View<super::Ui<'ui>, ()>> View<super::Ui<'_>, ()> for Selectable<V> {
    fn view(&mut self, ui: super::Ui) {
        ui.group(|ui|
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.selected, "");

                self.inner.view(ui);
            })
        );
    }
}

impl<'s, V: for<'ui> Table<super::Ui<'ui>, ()>> Table<super::Ui<'s>, ()> for Selectable<V> {
    type Title = <V as Table<super::Ui<'s>, ()>>::Title;

    type Child = <V as Table<super::Ui<'s>, ()>>::Child;

    fn title(&self) -> &Self::Title {
        self.inner.title()
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
impl<V: for<'ui> TableMut<super::Ui<'ui>, ()>> TableMut<super::Ui<'_>, ()> for Selectable<V> {
    fn insert(&mut self, at: usize, cell: Self::Child) {
        self.inner.insert(at, cell)
    }
}
