use crate::itemiser::Itemiser;

pub trait Occupied {
    fn ordinals(&self) -> &[usize];
    
    fn step(&mut self) -> bool;
    
    fn into_itemiser(self) -> IntoItemiser<Self> where Self: Sized {
        self.into()
    }
}

pub struct IntoItemiser<O: Occupied> {
    occupied: O,
    initial: bool,
}

impl<O: Occupied> From<O> for IntoItemiser<O> {
    fn from(occupied: O) -> Self {
        Self {
            occupied,
            initial: true,
        }
    }
}

impl<'a, I: Occupied> Itemiser for IntoItemiser<I> {
    type Item = [usize];

    fn next(&mut self) -> Option<&Self::Item> {
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