use std::mem;

pub trait Itemiser {
    type Item<'c> where Self: 'c;

    fn next<'c>(&'c mut self) -> Option<Self::Item<'c>>;

    fn into_iter_<I: ?Sized>(self) -> Iter<Self> where for <'any> Self: Itemiser<Item<'any> = &'any I>, I: ToOwned, Self: Sized + 'static {
        Iter {
            itemiser: self,
        }
    }

    // fn into_iter(self) -> Iter<Self, impl for <'c> Fn(Self::Item<'c>)> where Self: Sized {
    //     Iter::over(self, |item| {})
    // }

    // fn into_iter(self) -> Iter<Self, impl for <'c> Fn(&'c mut Self) -> Option<<Self::Item<'c> as ToOwned>::Owned>> where Self: Sized + 'static, for <'d> Self::Item<'d> : ToOwned {
    //     // Iter::over(self, for <'g> |itemiser: &'g mut Self| -> Option<<Self::Item<'g> as ToOwned>::Owned> {
    //     //     Self::cl(itemiser)
    //     // })
    //     Iter::over(self, Self::constrain(Self::cl))
    // }

    fn collect_<I>(self) -> Vec<I::Owned> where for <'any> Self: Itemiser<Item<'any> = &'any I>, I: ToOwned + ?Sized, Self: Sized + 'static {
        let mut items = vec![];
        self.for_each(|item| {
            let owned = item.to_owned();
            items.push(owned);
        });
        items
    }

    fn map_<V, U: ?Sized + 'static, F>(self, f: F) -> Map<Self, F, V> where F: FnMut(&U) -> V, for <'any> Self: Itemiser<Item<'any> = &'any U>, Self: Sized  {
        Map {
            itemiser: self,
            transform: f,
            next: None
        }
    }

    fn map_in_place<B, U, F>(self, buffer: B, f: F) -> MapInPlace<Self, F, B> where F: FnMut(&U, &mut B), for <'any> Self: Itemiser<Item<'any> = &'any U>, Self: Sized  {
        MapInPlace {
            itemiser: self,
            transform: f,
            buffer,
        }
    }

    fn for_each<F>(mut self, mut f: F) where F: FnMut(Self::Item<'_>), Self: Sized {
        while let Some(item) = self.next() {
            f(item)
        }
    }

    // fn find<'c, F>(&'c mut self, mut predicate: F) -> Option<Self::Item<'c>> where F: FnMut(&Self::Item<'_>) -> bool, Self: Sized {
    fn find<'c, I: ?Sized, F>(&'c mut self, predicate: &mut F) -> Option<Self::Item<'c>> where for <'any> Self: Itemiser<Item<'any> = &'any I> + 'static, F: FnMut(&I) -> bool, Self: Sized {
        while let Some(item) = self.next() {
            if predicate(&item) {
                let ptr: *const Self::Item<'_> = &item;
                // mem::forget(item); // only needed if item is an owned value
                // SAFETY: workaround for borrow checker limitation. Prevents `item` from holding a mutable
                // borrow on `self` beyond returning.
                let ptr = unsafe { mem::transmute::<_, *const Self::Item<'c>>(ptr) };
                let item = unsafe {std::ptr::read(ptr)};
                return Some(item)
            }
        }
        None
    }

    // fn filter_<F>(self, mut predicate: F) -> Filter<Self, F> where F: FnMut(&Self::Item<'_>) -> bool, Self: Sized {
    fn filter<I: ?Sized + 'static, F>(self, predicate: F) -> Filter<Self, F> where for <'any> Self: Itemiser<Item<'any> = &'any I> + 'static, F: FnMut(&I) -> bool, Self: Sized {
        Filter {
            itemiser: self,
            predicate,
        }
    }
}

pub struct Map<S, F, V> {
    itemiser: S,
    transform: F,
    next: Option<V>
}

impl<V, U: ?Sized + 'static, S, F> Itemiser for Map<S, F, V> where F: FnMut(&U) -> V, for <'any> S: Itemiser<Item<'any> = &'any U> + 'static {
    type Item<'c> = &'c V where Self: 'c;

    fn next<'c>(&'c mut self) -> Option<Self::Item<'c>> {
        self.next = self.itemiser.next().map(|item| (self.transform)(item));
        self.next.as_ref()
    }
}

pub struct Filter<S, F> {
    itemiser: S,
    predicate: F
}

impl<I: ?Sized + 'static, S, F> Itemiser for Filter<S, F> where for <'any> S: Itemiser<Item<'any> = &'any I> + 'static, F: FnMut(&I) -> bool + 'static {
    type Item<'c> = &'c I where Self: 'c;

    fn next<'c>(&'c mut self) -> Option<Self::Item<'c>> {
        self.itemiser.find(&mut self.predicate)
    }
}

pub struct Iter<S> {
    itemiser: S,
}

impl<I: ?Sized + ToOwned + 'static, S> Iterator for Iter<S> where for <'any> S: Itemiser<Item<'any> = &'any I> + 'static {
    type Item = I::Owned;

    fn next(&mut self) -> Option<Self::Item> {
        self.itemiser.next().map(|item| item.to_owned())
    }
}

pub struct SliceIt<'a, T> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T> From<&'a [T]> for SliceIt<'a, T> {
    fn from(slice: &'a [T]) -> Self {
        Self {
            slice,
            index: 0
        }
    }
}

impl<'a, T> Itemiser for SliceIt<'a, T> {
    type Item<'c> = &'c T where Self: 'c;

    fn next<'c>(&'c mut self) -> Option<Self::Item<'c>> {
        if self.index < self.slice.len() {
            let val = &self.slice[self.index];
            self.index += 1;
            Some(val)
        } else {
            None
        }
    }
}

pub struct MapInPlace<S, F, B> {
    itemiser: S,
    transform: F,
    buffer: B,
}

impl<B, U, S, F> Itemiser for MapInPlace<S, F, B> where F: FnMut(&U, &mut B), for <'any> S: Itemiser<Item<'any> = &'any U> + 'static {
    type Item<'c> = &'c B where Self: 'c;

    fn next<'c>(&'c mut self) -> Option<Self::Item<'c>> {
        match self.itemiser.next() {
            None => None,
            Some(item) => {
                (self.transform)(item, &mut self.buffer);
                Some(&self.buffer)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::itemiser::{Itemiser, SliceIt};

    #[test]
    fn slice_it() {
        let itemiser = SliceIt::from([0, 10, 20].as_slice());
        let collected = itemiser.collect_();
        assert_eq!(vec![0, 10, 20], collected);
    }

    #[test]
    fn map_in_place() {
        let itemiser = SliceIt::from([0, 10, 20].as_slice());
        let map = itemiser.map_in_place(0, |item, buffer| *buffer = *item * 10);
        assert_eq!(vec![0, 100, 200], map.collect_());
    }

    #[test]
    fn map() {
        let slice = [0, 10, 20].as_slice();
        let itemiser = SliceIt::from(slice);
        let map = itemiser.map_(|item| *item * 10);
        assert_eq!(vec![0, 100, 200], map.collect_());
    }

    #[test]
    fn filter() {
        let slice = [0, 1, 2, 3, 4, 5, 6].as_slice();
        let itemiser = SliceIt::from(slice);
        let filter = itemiser.filter(|item| *item % 2 == 0);
        assert_eq!(vec![0, 2, 4, 6], filter.collect_());
    }
    
    #[test]
    fn into_iter() {
        let slice = [0, 10, 20].as_slice();
        let itemiser = SliceIt::from(slice);
        let it = itemiser.into_iter_();
        let collected = it.collect::<Vec<_>>();
        assert_eq!(vec![0, 10, 20], collected);
    }
}