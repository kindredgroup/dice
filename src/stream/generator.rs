use crate::stream::itemiser::Itemiser;

pub trait Generator {
    type Item: ?Sized;
    
    fn read(&self) -> &Self::Item;
    
    fn advance(&mut self) -> bool;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    #[inline]
    fn into_itemiser(self) -> IntoItemiser<Self> where Self: Sized {
        self.into()
    }
}

pub struct IntoItemiser<G: Generator> {
    generator: G,
    initial: bool,
}

impl<G: Generator> From<G> for IntoItemiser<G> {
    #[inline]
    fn from(occupied: G) -> Self {
        Self {
            generator: occupied,
            initial: true,
        }
    }
}

impl<G: Generator> Itemiser for IntoItemiser<G> {
    type Item = G::Item;

    #[inline]
    fn next(&mut self) -> Option<&Self::Item> {
        let has_more = if self.initial {
            self.initial = false;
            true
        } else {
            self.generator.advance()
        };
        if has_more {
            Some(self.generator.read())
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.generator.size_hint()
    }
}