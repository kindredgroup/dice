use crate::itemiser::Itemiser;

pub trait Generator {
    type Item: ?Sized;
    
    fn read(&self) -> &Self::Item;
    
    fn advance(&mut self) -> bool;

    #[inline]
    fn into_itemiser(self) -> IntoItemiser<Self> where Self: Sized {
        self.into()
    }
}

pub struct IntoItemiser<G: Generator> {
    occupied: G,
    initial: bool,
}

impl<G: Generator> From<G> for IntoItemiser<G> {
    #[inline]
    fn from(occupied: G) -> Self {
        Self {
            occupied,
            initial: true,
        }
    }
}

impl<'a, G: Generator> Itemiser for IntoItemiser<G> {
    type Item = G::Item;

    #[inline]
    fn next(&mut self) -> Option<&Self::Item> {
        let has_more = if self.initial {
            self.initial = false;
            true
        } else {
            self.occupied.advance()
        };
        if has_more {
            Some(self.occupied.read())
        } else {
            None
        }
    }
}