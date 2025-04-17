use std::fmt::Formatter;

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
    fn push(&mut self, item: T);

    #[inline]
    fn push_all(&mut self, items: impl IntoIterator<Item = T>) {
        for item in items {
            self.push(item)
        }
    }
}

impl<V: VecWrapperMut<Item = T>, T> Push<T> for V {
    #[inline]
    fn push(&mut self, item: T) {
        self.vec_mut().push(item)
    }
}

pub trait IntoInner {
    type Item;

    fn into_inner(self) -> Vec<Self::Item>;
}

pub trait New<T> {
    fn new(items: impl IntoIterator<Item = T>) -> Self;

    fn with_capacity(capacity: usize) -> Self;
}

impl<C, T> New<T> for C
where
    C: Default + VecWrapperMut<Item = T>,
{
    #[inline]
    fn new(items: impl IntoIterator<Item = T>) -> Self {
        let iterator = items.into_iter();
        let (min_items, _) = iterator.size_hint();
        let mut container = Self::with_capacity(min_items);
        container.push_all(iterator);
        container
    }

    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        let mut container = Self::default();
        container.vec_mut().reserve(capacity);
        container
    }
}

fn format_elements<
    T,
    E: Fn(&T, &mut Formatter) -> std::fmt::Result,
    S: Fn(&mut Formatter) -> std::fmt::Result,
>(
    slice: &[T],
    format_element: E,
    format_separator: S,
    f: &mut Formatter<'_>,
) -> std::fmt::Result {
    for element in &slice[..slice.len() - 1] {
        format_element(element, f)?;
        format_separator(f)?;
    }
    format_element(&slice[slice.len() - 1], f)
}
