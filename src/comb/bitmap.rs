use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, Index, IndexMut};

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Bitmap(Vec<bool>);

impl Bitmap {
    #[inline]
    pub fn empty(len: usize) -> Self {
        Self(vec![false; len])
    }

    #[inline]
    pub fn full(len: usize) -> Self {
        Self(vec![true; len])
    }
    
    #[inline]
    pub fn size(&self) -> usize {
        self.0.iter().filter(|b| **b).count()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|&b| !b)
    }
    
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    #[inline]
    pub fn ordinals(&self) -> Iter {
        Iter::new(self)
    }
    
    #[inline]
    pub fn next_occupied(&self, start: usize) -> Option<usize> {
        for i in start..self.0.len() {
            if self.0[i] {
                return Some(i);
            }
        }
        None
    }
}

impl<I: IntoIterator<Item = usize>> From<(I, usize)> for Bitmap {
    #[inline]
    fn from(ordinals_and_len: (I, usize)) -> Self {
        let mut bitmap = Self::empty(ordinals_and_len.1);
        for ordinal in ordinals_and_len.0 {
            bitmap[ordinal] = true;
        }
        bitmap
    }
}

pub struct Iter<'a> {
    bitmap: &'a Bitmap,
    index: Option<usize>
}

impl<'a> Iter<'a> {
    #[inline]
    pub fn new(bitmap: &'a Bitmap) -> Self {
        Self {
            bitmap,
            index: bitmap.next_occupied(0),
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            None => None,
            Some(index) => {
                self.index = self.bitmap.next_occupied(index + 1);
                Some(index)
            }
        }
    }
}

impl From<Vec<bool>> for Bitmap {
    #[inline]
    fn from(backing: Vec<bool>) -> Self {
        Self(backing)
    }
}

impl Index<usize> for Bitmap {
    type Output = bool;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Bitmap {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Deref for Bitmap {
    type Target = [bool];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Bitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bitmap{:?}", self.ordinals().collect::<Vec<_>>())
    }
}

impl Display for Bitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ordinals().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::bitmap::Bitmap;

    #[test]
    fn get_set() {
        let mut bitmap = Bitmap::empty(3);
        assert!(!bitmap[0]);
        bitmap[0] = true;
        assert!(bitmap[0]);
    }
    
    #[test]
    fn is_empty() {
        let mut bitmap = Bitmap::empty(2);
        assert!(bitmap.is_empty());
        bitmap[1] = true;
        assert!(!bitmap.is_empty());
        
        let mut bitmap = Bitmap::full(2);
        assert!(!bitmap.is_empty());
        bitmap[0] = false;
        assert!(!bitmap.is_empty());
        bitmap[1] = false;
        assert!(bitmap.is_empty());
    }

    #[test]
    fn debug() {
        let bitmap = Bitmap::from(vec![false, true, false, true]);
        assert_eq!("Bitmap[1, 3]", format!("{bitmap:?}"));
    }
    
    #[test]
    fn display() {
        let bitmap = Bitmap::from(vec![false, true, false, true]);
        assert_eq!("[1, 3]", format!("{bitmap}"));
    }
    
    #[test]
    fn from_ordinals() {
        let bitmap = Bitmap::from(([1, 3], 4));
        assert_eq!(Bitmap::from(vec![false, true, false, true]), bitmap);
    }
    
    #[test]
    fn ordinals_empty() {
        let bitmap = Bitmap::empty(3);
        let mut it = bitmap.ordinals();
        assert_eq!(None, it.next());
    }

    #[test]
    fn ordinals_non_empty() {
        let bitmap = Bitmap::from(([1, 3], 4));
        let mut it = bitmap.ordinals();
        assert_eq!(Some(1), it.next());
        assert_eq!(Some(3), it.next());
        assert_eq!(None, it.next());
    }
}