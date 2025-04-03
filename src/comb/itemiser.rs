pub trait Itemiser {
    type Item<'c> where Self: 'c;
    
    fn next<'c>(&'c mut self) -> Option<Self::Item<'c>>;
    
    // fn into_iter<'c>(self) -> Iter<Self, Self::Item<'c>, <Self::Item<'c> as ToOwned>::Owned> where Self: Sized + 'static, for <'d> Self::Item<'d>: ToOwned {
    //     Iter::from(self)
    // }

    // fn into_iter(self) -> Iter<Self, impl for <'c> Fn(Self::Item<'c>)> where Self: Sized {
    //     Iter::over(self, |item| {})
    // }
    
    // fn collect<'c>(&mut self) -> Vec<<Self::Item<'c> as ToOwned>::Owned> where Self::Item<'c> : ToOwned {
    //     let mut items = vec![];
    //     while let Some(item) = self.next() {
    //         items.push(item.to_owned());
    //     }
    //     items
    // }
    
    fn for_each<F>(mut self, mut f: F) where F: FnMut(Self::Item<'_>), Self: Sized {
        while let Some(item) = self.next() {
            f(item)
        }
    }
    
    // fn collect(self) where for <'any> Self::Item<'any> : ToOwned, Self: Sized + 'static, for <'any> <Self::Item<'any> as ToOwned>::Owned: 'static {
    //     let mut items = vec![];
    //     self.for_each(|item| {
    //         let owned = Self::into_owned(item);
    //         // items.push(owned);
    //     });
    // }

    // fn collect_slice<T>(self) -> Vec<Vec<T>> where for <'any> Self: Itemiser<Item<'any> = &'any [T]>, T: Clone, Self: Sized + 'static {
    //     let mut items = vec![];
    //     self.for_each(|item| {
    //         let owned = item.to_owned();
    //         items.push(owned);
    //     });
    //     items
    // }
    // 
    // fn collect_clone<T>(self) -> Vec<T> where for <'any> Self: Itemiser<Item<'any> = &'any T>, T: Clone, Self: Sized + 'static {
    //     let mut items = vec![];
    //     self.for_each(|item| {
    //         let owned = item.to_owned();
    //         items.push(owned);
    //     });
    //     items
    // }

    // fn into_iter(self) -> Iter<Self, impl for <'c> Fn(&'c mut Self) -> Option<<Self::Item<'c> as ToOwned>::Owned>> where Self: Sized + 'static, for <'d> Self::Item<'d> : ToOwned {
    //     // Iter::over(self, for <'g> |itemiser: &'g mut Self| -> Option<<Self::Item<'g> as ToOwned>::Owned> {
    //     //     Self::cl(itemiser)
    //     // })
    //     Iter::over(self, Self::constrain(Self::cl))
    // }
    // 
    // fn constrain<S: Itemiser, F>(f: F) -> F where for <'c> F: Fn(&'c mut S) -> Option<<Self::Item<'c> as ToOwned>::Owned>, for<'c> <Self as Itemiser>::Item<'c>: ToOwned {
    //     f
    // }
    // 
    // fn cl<'g>(itemiser: &'g mut Self) ->  Option<<Self::Item<'g> as ToOwned>::Owned> where Self: Sized + 'static, for <'d> Self::Item<'d> : ToOwned {
    //     None
    // }
    // 
    // fn cl1<'g>(itemiser: &mut Self) ->  Option<<Self::Item<'g> as ToOwned>::Owned> where Self: Sized + 'static, for <'d> Self::Item<'d> : ToOwned {
    //     None
    // }

    fn ccc<T>(self) -> Vec<T::Owned> where for <'any> Self: Itemiser<Item<'any> = &'any T>, T: ToOwned + ?Sized, Self: Sized + 'static {
        let mut items = vec![];
        self.for_each(|item| {
            let owned = item.to_owned();
            items.push(owned);
        });
        items
    }

    // where F: FnMut(U) -> V, for <'any> S: Itemiser<Item<'any> = U> + 'static

    fn map<V, U, F>(self, f: F) -> Map<Self, F, U> where F: FnMut(&U) -> V, for <'any> Self: Itemiser<Item<'any> = &'any U>, Self: Sized  {
        Map {
            itemiser: self,
            transform: f,
            next: None
        }
    }
    // fn map<V, F>(self, f: F) -> Map<Self, impl FnMut(Self::Item<'_>) -> V> where F: FnMut(Self::Item<'_>) -> V, Self: Sized {
    //     Map {
    //         itemiser: self,
    //         transform: f,
    //     }
    // }
}

pub struct Map<S, F, V> {
    itemiser: S,
    transform: F,
    next: Option<V>
}

impl<V, U, S, F> Itemiser for Map<S, F, V> where F: FnMut(&U) -> V, for <'any> S: Itemiser<Item<'any> = &'any U> + 'static {
    type Item<'c> = &'c V where Self: 'c;

    fn next<'c>(&'c mut self) -> Option<Self::Item<'c>> {
        self.next = self.itemiser.next().map(|item| (self.transform)(item));
        self.next.as_ref()
        // match self.itemiser.next() {
        //     None => None,
        //     Some(item) => {
        //         let transformed = (self.transform)(item);
        //         Some(transformed)
        //     }
        // }
    }
}

#[cfg(test)]
mod tests2 {
    use crate::comb::itemiser::{Itemiser, SliceIt};

    #[test]
    fn map() {
        let it = SliceIt::from([0, 10, 20].as_slice());
        let map = it.map::<i32, i32, _>(|item| *item * 10);
        assert_eq!(vec![0, 100, 200], map.ccc());
    }
}

// fn dummy<S: Itemiser, W, F>(f: F) -> F where F: for<'c> Fn(&'c mut S) -> Option<W> {
//     f
// }

// pub struct Iter<'c, I: Itemiser> {
//     itemiser: I,
//     __phantom_data: PhantomData<&'c ()>
// }
// 
// impl<'c, I: ToOwned, S: Itemiser<Item<'c> = I> + 'static> From<S> for Iter<'c, S> {
//     fn from(itemiser: S) -> Self {
//         Self {
//             itemiser,
//             __phantom_data: Default::default(),
//         }
//     }
// }
// 
// impl<'c, I: ToOwned, S: Itemiser<Item<'c> = I> + 'static> Iterator for Iter<'c, S> {
//     type Item = I::Owned;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         self.itemiser.next().map(|item| item.to_owned())
//     }
// }

// pub struct Iter<S: Itemiser, F> {
//     itemiser: S,
//     f: F
// }
// 
// impl<W, S: Itemiser, F> Iter<S, F> where F: for <'e> Fn(&'e mut S) -> Option<W> {
//     pub fn over(itemiser: S, f: F) -> Self {
//         Self {
//             itemiser,
//             f,
//         }
//     }
// }
// 
// impl<W, S: Itemiser, F> Iterator for Iter<S, F> where F: for <'c> Fn(&'c mut S) -> Option<W> {
//     type Item = W;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// impl<I, S: for <'e> Itemiser<Item<'e> = I>, F> Iter<S, F> where F: Fn(I) {
//     fn over(itemiser: S, f: F) -> Self {
//         Self {
//             itemiser,
//             f,
//         }
//     }
// }

// impl<W, S: Itemiser + 'static> Iterator for Iter<S, W> where for <'c> S::Item<'c>: ToOwned {
//     type Item = W;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         
//         // self.itemiser.next().map(|item| item.to_owned())
//         todo!()
//     }
// }

//---

// 
// pub struct Iter<S: Itemiser, I, W> {
//     itemiser: S,
//     __phantom_data: PhantomData<(I, W)>
// }
// 
// impl<I: ToOwned, S: Itemiser + 'static> From<S> for Iter<S, I, I::Owned> where for <'e> S::Item<'e> : ToOwned {
//     fn from(itemiser: S) -> Self {
//         Self {
//             itemiser,
//             __phantom_data: Default::default(),
//         }
//     }
// }
// 
// impl<'c, I: ToOwned, S: Itemiser<Item<'c> = I> + 'static> Iterator for Iter<S, I, I::Owned> {
// // impl<I: ToOwned, S: for <'c> Itemiser<Item<'c> = I> + 'static> Iterator for Iter<S, I, I::Owned> {
//     type Item = I::Owned;
// 
//     fn next<'a>(&'a mut self) -> Option<Self::Item> {
//         let i: &'a mut S = &mut self.itemiser;
//         let g: &'c Option<I> = &i.next();
//         // let g = &mut i.next() as *mut Option<I>;
//         // let next: Option<I> = unsafe { *g };
//         // self.itemiser.next().map(|item| item.to_owned())
//         todo!()
//     }
// }


//---

// impl<I: ToOwned, S: for <'e> Itemiser<Item<'e> = I> + 'static> From<S> for Iter<S> {
//     fn from(itemiser: S) -> Self {
//         Self {
//             itemiser,
//         }
//     }
// }

// impl<I: ToOwned, S: for <'c> Itemiser<Item<'c> = I> + 'static> Iterator for Iter<S> {
//     type Item = I::Owned;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         self.itemiser.next().map(|item| item.to_owned())
//     }
// }

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

#[cfg(test)]
mod tests {
    use crate::comb::itemiser::{Itemiser, SliceIt};

    #[test]
    fn slice_it() {
        let it = SliceIt::from([0, 10, 20].as_slice());
        let collected = it.ccc();
        assert_eq!(vec![0, 10, 20], collected);
    }
}