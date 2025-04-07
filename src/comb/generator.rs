use crate::itemiser::Itemiser;

pub trait Generator {
    fn ordinals(&self) -> &[usize];
    
    fn advance(&mut self) -> bool;

    #[inline]
    fn into_itemiser(self) -> IntoItemiser<Self> where Self: Sized {
        self.into()
    }
}

pub struct IntoItemiser<O: Generator> {
    occupied: O,
    initial: bool,
}

impl<O: Generator> From<O> for IntoItemiser<O> {
    #[inline]
    fn from(occupied: O) -> Self {
        Self {
            occupied,
            initial: true,
        }
    }
}

impl<'a, I: Generator> Itemiser for IntoItemiser<I> {
    type Item = [usize];

    #[inline]
    fn next(&mut self) -> Option<&Self::Item> {
        let has_more = if self.initial {
            self.initial = false;
            true
        } else {
            self.occupied.advance()
        };
        if has_more {
            Some(self.occupied.ordinals())
        } else {
            None
        }
    }
}