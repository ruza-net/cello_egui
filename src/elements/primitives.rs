use crate::defaults::*;


pub struct Nothing;
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
