pub trait Itemiser {
    type Item: ?Sized;

    fn next(&mut self) -> Option<&Self::Item>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    #[inline]
    fn into_iter(self) -> Iter<Self> where Self::Item: ToOwned, Self: Sized {
        Iter {
            itemiser: self,
        }
    }

    #[inline]
    fn collect(mut self) -> Vec<<Self::Item as ToOwned>::Owned> where Self::Item: ToOwned, Self: Sized {
        let mut items = vec![];
        while let Some(item) = self.next() {
            items.push(item.to_owned())
        }
        items
    }

    #[inline]
    fn map_owned<V, F>(self, f: F) -> MapOwned<Self, F, V> where F: FnMut(&Self::Item) -> V, Self: Sized {
        MapOwned {
            itemiser: self,
            transform: f,
            next: None
        }
    }

    #[inline]
    fn map_borrowed<B, F>(self, buffer: &mut B, f: F) -> MapBorrowed<Self, F, B> where F: FnMut(&Self::Item, &mut B), Self: Sized  {
        MapBorrowed {
            itemiser: self,
            transform: f,
            buffer,
        }
    }

    #[inline]
    fn for_each<F>(mut self, mut f: F) where F: FnMut(&Self::Item), Self: Sized {
        while let Some(item) = self.next() {
            f(item)
        }
    }

    #[inline]
    fn find<F>(&mut self, predicate: &mut F) -> Option<&Self::Item> where F: FnMut(&Self::Item) -> bool {
        while let Some(item) = self.next() {
            if predicate(item) {
                // SAFETY: workaround for a borrow checker limitation. Prevents `item` from holding a mutable
                // borrow on `self` if discarded by the predicate check.
                return Some(unsafe { expand_lifetime(item) })
            }
        }
        None
    }

    #[inline]
    fn filter<F>(self, predicate: F) -> Filter<Self, F> where  F: FnMut(&Self::Item) -> bool, Self: Sized {
        Filter {
            itemiser: self,
            predicate,
        }
    }
}

#[inline(always)]
unsafe fn expand_lifetime<'long, T: ?Sized>(v: &T) -> &'long T {
    unsafe {
        &*(v as *const T)
    }
}

pub struct Iter<S> {
    itemiser: S,
}

impl<S> Iterator for Iter<S> where S: Itemiser, S::Item: ToOwned {
    type Item = <S::Item as ToOwned>::Owned;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.itemiser.next().map(ToOwned::to_owned)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.itemiser.size_hint()
    }
}

pub struct MapOwned<S, F, V> {
    itemiser: S,
    transform: F,
    next: Option<V>
}

impl<V, S, F> Itemiser for MapOwned<S, F, V> where F: FnMut(&S::Item) -> V, S: Itemiser {
    type Item = V where;

    #[inline]
    fn next(&mut self) -> Option<&Self::Item> {
        self.next = self.itemiser.next().map(|item| (self.transform)(item));
        self.next.as_ref()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.itemiser.size_hint()
    }
}

pub struct MapBorrowed<'b, S, F, B> {
    itemiser: S,
    transform: F,
    buffer: &'b mut B,
}

impl<B, S, F> Itemiser for MapBorrowed<'_, S, F, B> where F: FnMut(&S::Item, &mut B), S: Itemiser {
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<&Self::Item> {
        match self.itemiser.next() {
            None => None,
            Some(item) => {
                (self.transform)(item, self.buffer);
                Some(self.buffer)
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.itemiser.size_hint()
    }
}

pub struct Filter<S, F> {
    itemiser: S,
    predicate: F
}

impl<S, F> Itemiser for Filter<S, F> where S: Itemiser, F: FnMut(&S::Item) -> bool {
    type Item = S::Item;

    #[inline]
    fn next(& mut self) -> Option<&Self::Item> {
        self.itemiser.find(&mut self.predicate)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.itemiser.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

pub struct SliceIt<'a, T> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T> From<&'a [T]> for SliceIt<'a, T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        Self {
            slice,
            index: 0
        }
    }
}

impl<T> Itemiser for SliceIt<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<&Self::Item> {
        if self.index < self.slice.len() {
            let val = &self.slice[self.index];
            self.index += 1;
            Some(val)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.slice.len();
        (len, Some(len))
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::itemiser::{Itemiser, SliceIt};

    #[test]
    fn into_iter_empty() {
        let slice: &[i32] = &[];
        let itemiser = SliceIt::from(slice);
        let it = itemiser.into_iter();
        assert_eq!((0, Some(0)), it.size_hint());
        let collected = it.collect::<Vec<_>>();
        assert_eq!(Vec::<i32>::new(), collected);
    }

    #[test]
    fn into_iter_occupied() {
        let slice = [0, 10, 20].as_slice();
        let itemiser = SliceIt::from(slice);
        let it = itemiser.into_iter();
        assert_eq!((3, Some(3)), it.size_hint());
        let collected = it.collect::<Vec<_>>();
        assert_eq!(vec![0, 10, 20], collected);
    }

    #[test]
    fn slice_itemiser_and_collect() {
        let itemiser = SliceIt::from([0, 10, 20].as_slice());
        assert_eq!((3, Some(3)), itemiser.size_hint());
        let collected = itemiser.collect();
        assert_eq!(vec![0, 10, 20], collected);
    }

    #[test]
    fn for_each() {
        let slice = [0, 10, 20].as_slice();
        let itemiser = SliceIt::from(slice);
        let mut collected = vec![];
        itemiser.for_each(|item| {
            collected.push(item.to_owned());
        });
        assert_eq!(vec![0, 10, 20], collected);
    }

    #[test]
    fn map_owned() {
        let slice = [0, 10, 20].as_slice();
        let itemiser = SliceIt::from(slice);
        let mut invocations = 0;
        let map = itemiser.map_owned(|item| {
            invocations += 1;
            *item * 10
        });
        assert_eq!((3, Some(3)), map.size_hint());
        assert_eq!(vec![0, 100, 200], map.collect());
        assert_eq!(3, invocations);
    }

    #[test]
    fn map_borrowed() {
        let itemiser = SliceIt::from([0, 10, 20].as_slice());
        let mut invocations = 0;
        let mut buffer = 0;
        let map = itemiser.map_borrowed(&mut buffer, |item, buffer| {
            invocations += 1;
            *buffer = *item * 10
        });
        assert_eq!((3, Some(3)), map.size_hint());
        assert_eq!(vec![0, 100, 200], map.collect());
        assert_eq!(3, invocations);
        assert_eq!(200, buffer);
    }

    #[test]
    fn filter() {
        let slice = [0, 1, 2, 3, 4, 5, 6].as_slice();
        let itemiser = SliceIt::from(slice);
        let mut invocations = 0;
        let filter = itemiser.filter(|item| {
            invocations += 1;
            *item % 2 == 0
        });
        assert_eq!((0, Some(7)), filter.size_hint());
        assert_eq!(vec![0, 2, 4, 6], filter.collect());
        assert_eq!(7, invocations);
    }

    #[test]
    fn filter_then_map() {
        let slice = [0, 1, 2, 3, 4, 5, 6].as_slice();
        let itemiser = SliceIt::from(slice);
        let mut filter_invocations = 0;
        let filter = itemiser.filter(|item| {
            filter_invocations += 1;
            *item % 2 == 0
        });
        let mut map_invocations = 0;
        let map = filter.map_owned(|item| {
            map_invocations += 1;
            item * 10
        });
        assert_eq!(vec![0, 20, 40, 60], map.collect());
        assert_eq!(7, filter_invocations);
        assert_eq!(4, map_invocations);
    }
}