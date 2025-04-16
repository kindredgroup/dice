use std::fmt::Display;

pub mod con;
pub mod dis;

trait VecWrapper {
    type Item;

    fn vec(&self) -> &Vec<Self::Item>;
}

trait VecWrapperMut: VecWrapper {
    fn vec_mut(&mut self) -> &mut Vec<Self::Item>;
}

pub trait Push<T> {
    fn push(&mut self, value: T);
}

impl<V: VecWrapperMut<Item=T>, T> Push<T> for V {
    #[inline]
    fn push(&mut self, value: T) {
        self.vec_mut().push(value)
    }
}

pub trait Length {
    fn len(&self) -> usize;
    
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<V: VecWrapper> Length for V {
    #[inline]
    fn len(&self) -> usize {
        self.vec().len()
    }
}

pub trait New<C, T> {
    fn new(items: impl IntoIterator<Item=T>) -> C;
}

impl<C, T> New<C, T> for C where C: Default + VecWrapperMut<Item=T> {
    fn new(items: impl IntoIterator<Item=T>) -> C {
        let mut c = C::default();
        let iterator = items.into_iter();
        let (min_items, _) = iterator.size_hint();
        c.vec_mut().reserve(min_items);
        for item in iterator {
            c.push(item)
        }
        c
    }
}

fn join_display_elements<T: Display>(slice: &[T], sep: &'static str) -> String {
    let strings = slice.iter().map(ToString::to_string).collect::<Vec<_>>();
    strings.join(sep)
}