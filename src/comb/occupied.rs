use std::marker::PhantomData;
use crate::comb::itemiser::Itemiser;

pub trait Occupied {
    fn ordinals(&self) -> &[usize];
    
    fn step(&mut self) -> bool;
    
    // fn into_iter(self) -> Iter<Self> where Self: Sized {
    //     self.into()
    // }
    
    fn into_itemiser(self) -> IntoItemiser<Self> where Self: Sized {
        self.into()
    }
}

// pub struct Iter<O: Occupied> {
//     occupied: O,
//     initial: bool,
// }
// 
// impl<O: Occupied> From<O> for Iter<O> {
//     fn from(occupied: O) -> Self {
//         Self {
//             occupied, initial: true
//         }
//     }
// }
// 
// impl<O: Occupied> Iterator for Iter<O> {
//     type Item = Vec<usize>;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         let has_more = if self.initial {
//             self.initial = false;
//             true
//         } else {
//             self.occupied.step()
//         };
//         if has_more {
//             let ordinals = self
//                 .occupied
//                 .ordinals()
//                 .iter()
//                 .copied()
//                 .collect();
//             Some(ordinals)
//         } else {
//             None
//         }
//     }
// }

pub struct IntoItemiser<O: Occupied> {
    occupied: O,
    initial: bool,
    // __phantom_data: PhantomData<&'a ()>
}

impl<O: Occupied> From<O> for IntoItemiser<O> {
    fn from(occupied: O) -> Self {
        Self {
            occupied,
            initial: true,
            // __phantom_data: Default::default(),
        }
    }
}

impl<'a, I: Occupied> Itemiser for IntoItemiser<I> {
    type Item<'c> = &'c [usize] where Self: 'c;

    fn next<'c>(&'c mut self) -> Option<Self::Item<'c>> {
        let has_more = if self.initial {
            self.initial = false;
            true
        } else {
            self.occupied.step()
        };
        if has_more {
            Some(self.occupied.ordinals())
        } else {
            None
        }
    }
}