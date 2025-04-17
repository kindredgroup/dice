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

pub trait IntoInner {
    type Item;
    
    fn into_inner(self) -> Vec<Self::Item>;
}

// pub trait Length {
//     fn len(&self) -> usize;
//     
//     #[inline]
//     fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
// }

// impl<V: VecWrapper> Length for V {
//     #[inline]
//     fn len(&self) -> usize {
//         self.vec().len()
//     }
// }

pub trait New<T> {
    fn new(items: impl IntoIterator<Item=T>) -> Self;
    
    fn with_capacity(capacity: usize) -> Self;
}

impl<C, T> New<T> for C where C: Default + VecWrapperMut<Item=T> {
    #[inline]
    fn new(items: impl IntoIterator<Item=T>) -> Self {
        let iterator = items.into_iter();
        let (min_items, _) = iterator.size_hint();
        let mut container = C::with_capacity(min_items);
        for item in iterator {
            container.push(item)
        }
        container
    }

    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        let mut container = C::default();
        container.vec_mut().reserve(capacity);
        container
    }
}

fn join_display_elements<T: Display>(slice: &[T], sep: &'static str) -> String {
    let strings = slice.iter().map(ToString::to_string).collect::<Vec<_>>();
    strings.join(sep)
}