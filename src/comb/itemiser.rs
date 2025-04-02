pub trait Itemiser {
    fn next(&mut self) -> Option<&[usize]>;
    
    fn into_iter(self) -> Iter<Self> where Self: Sized {
        self.into()
    }
}

pub struct Iter<I: Itemiser> {
    itemiser: I,
}

impl<'a, I: Itemiser> From<I> for Iter<I> {
    fn from(itemiser: I) -> Self {
        Self {
            itemiser
        }
    }
}

impl<I: Itemiser> Iterator for Iter<I> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.itemiser.next().map(|ordinals| ordinals.iter().copied().collect::<Vec<_>>())
    }
}
