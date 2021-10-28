use crate::elements::{ BoxTable, Label, Ui };

#[macro_export]
macro_rules! fill {
    () => {
        Default::default()
    };
}


pub trait Boxed {
    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
}
impl<T> Boxed for T {}

pub trait Labeled {
    fn label(self) -> Label<Self> where Self: Sized;
}
impl<S> Labeled for S {
    fn label(self) -> Label<Self>
    where Self: Sized {
        Label(self)
    }
}

pub trait TableMut<In, Out>: cello_model::Table<In, Out> {
    fn insert(&mut self, at: usize, cell: Self::Child);

    fn push(&mut self, cell: Self::Child) {
        self.insert(self.len() - 1, cell)
    }
}

pub trait AsBoxTable<T>: for<'ui> TableMut<Ui<'ui>, (), Title = T, Child = BoxTable<T>> {
    fn as_box_table(self) -> BoxTable<T>;
}
impl<T, X: 'static + for<'ui> TableMut<Ui<'ui>, (), Title = T, Child = BoxTable<T>>> AsBoxTable<T> for X {
    fn as_box_table(self) -> BoxTable<T> {
        BoxTable::from(Box::new(self) as Box<dyn for<'ui> TableMut<Ui<'ui>, (), Child = BoxTable<T>, Title = T>>)
    }
}
