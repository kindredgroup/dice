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
    pub fn ordinals(&self) -> Vec<usize> {
        self.0.iter().enumerate().filter(|(_, flag)| **flag).map(|(ordinal, _)| ordinal).collect()
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

// impl<'a, I: Iterator<Item = &'a usize>> From<(I, usize)> for Bitmap {
//     #[inline]
//     fn from(ordinals_and_len: (I, usize)) -> Self {
//         let mut bitmap = Self::empty(ordinals_and_len.1);
//         for ordinal in ordinals_and_len.0 {
//             bitmap[*ordinal] = true;
//         }
//         bitmap
//     }
// }

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
        write!(f, "Bitmap{:?}", self.ordinals())
    }
}

impl Display for Bitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ordinals())
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
}