pub trait Itemiser {
    fn ordinals(&self) -> &[usize];
    
    fn step(&mut self) -> bool;
    
    fn iter(&mut self) -> Iter<Self> where Self: Sized {
        Iter::over(self)
    }
}

pub struct Iter<'a, I: Itemiser> {
    itemiser: &'a mut I,
    initial: bool,
}

impl<'a, I: Itemiser> Iter<'a, I> {
    pub fn over(itemiser: &'a mut I) -> Self {
        Iter {
            itemiser, initial: true
        }
    }
}

impl<I: Itemiser> Iterator for Iter<'_, I> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let has_more = if self.initial {
            self.initial = false;
            true
        } else {
            self.itemiser.step()
        };
        if has_more {
            let ordinals = self
                .itemiser
                .ordinals()
                .iter()
                .copied()
                .collect();
            Some(ordinals)
        } else {
            None
        }
    }
}
