//! An implementation of a combiner for the special case of <sup>n</sup>C<sub>n-1</sub>. It is
//! more efficient than the general-case <sup>n</sup>C<sub>r</sub> 
//! [`Combiner`](super::combiner::Combiner) implementation. 
//! 
//! For each yielded combination, the split-combiner also discloses the ordinal that was omitted 
//! from the combination. For example, in a <sup>4</sup>C<sub>3</sub> traversal, the first
//! combination is `Split([0, 1, 2], 3)`, succeeded by `Split([0, 1, 3], 2)`.

use std::ops::Deref;
use crate::capture::CaptureMut;

#[derive(Debug)]
pub struct SplitCombiner<'a> {
    ordinals: CaptureMut<'a, Vec<usize>, [usize]>,
    borrowed: Split<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Split<'a>(pub &'a [usize], pub usize);

impl<'a> SplitCombiner<'a> {
    #[inline]
    pub fn alloc(n: usize) -> CaptureMut<'a, Vec<usize>, [usize]> {
        debug_assert!(n > 0, "n must be greater than 0");
        vec![0; n - 1].into()
    }
    
    #[inline]
    pub fn new(n: usize) -> Self {
        Self::new_no_alloc(Self::alloc(n))
    }
    
    #[inline]
    pub fn new_no_alloc(mut ordinals: CaptureMut<'a, Vec<usize>, [usize]>) -> Self {
        let ordinals_len = ordinals.len();
        for ordinal in 0..ordinals_len {
            ordinals[ordinal] = ordinal;
        }
        
        // SAFETY: ordinals is either a Vec<usize> or a [usize] slice. 
        // In the case of an owned Vec, it is never resized by SplitCombiner (although it can
        // be moved if the encompassing SplitCombiner is moved). However, the address 
        // of the data is fixed even if the Vec is moved. 
        // In the case of a borrowed slice, the underlying Vec cannot be resized or moved due 
        // to the holding of a mutable reference for the lifetime of Self.
        let ordinals_borrow = unsafe { &*(ordinals.deref() as *const _)};
        Self {
            ordinals,
            borrowed: Split(ordinals_borrow, ordinals_len),
        }
    }
    
    #[inline]
    pub fn split(&self) -> &Split {
        &self.borrowed
    }

    #[inline]
    pub fn advance(&mut self) -> bool {
        if self.borrowed.1 != 0 {
            self.borrowed.1 -= 1;
            let mut index = 0;
            for ordinal in 0..self.ordinals.len() + 1 {
                if ordinal != self.borrowed.1 {
                    self.ordinals[index] = ordinal;
                    index += 1;
                }
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::capture::CaptureMut;
    use crate::comb::split_combiner::{Split, SplitCombiner};

    fn collect_splits(mut splitter: SplitCombiner) -> Vec<(Vec<usize>, usize)> {
        let mut outputs = vec![];
        loop {
            let Split(ordinals, omitted) = splitter.split();
            outputs.push((ordinals.to_owned().to_owned(), *omitted));
            if !splitter.advance() {
                break;
            }
        }
        outputs
    }

    #[test]
    fn split_1n0() {
        let outputs = collect_splits(SplitCombiner::new(1));
        println!("outputs: {outputs:?}");
        let expected = vec![
            (vec![], 0),
        ];
        assert_eq!(expected, outputs);
    }

    #[test]
    fn split_4n3() {
        let outputs = collect_splits(SplitCombiner::new(4));
        println!("outputs: {outputs:?}");
        let expected = vec![
            (vec![0, 1, 2], 3),
            (vec![0, 1, 3], 2),
            (vec![0, 2, 3], 1),
            (vec![1, 2, 3], 0),
        ];
        assert_eq!(expected, outputs);
    }
    
    #[test]
    fn safety_move_owned() {
        let mut combiners: [Option<SplitCombiner>; 2] = [const { None }; 2];
        combiners[0] = Some(SplitCombiner::new(4));
        assert_eq!(&Split(&[0, 1, 2], 3), combiners[0].as_ref().unwrap().split());
        
        combiners[1] = combiners[0].take();
        assert_eq!(&Split(&[0, 1, 2], 3), combiners[1].as_ref().unwrap().split());
        
        assert!(combiners[1].as_mut().unwrap().advance());
        assert_eq!(&Split(&[0, 1, 3], 2), combiners[1].as_ref().unwrap().split());

        assert!(combiners[1].as_mut().unwrap().advance());
        assert_eq!(&Split(&[0, 2, 3], 1), combiners[1].as_ref().unwrap().split());

        combiners[0] = combiners[1].take();
        assert_eq!(&Split(&[0, 2, 3], 1), combiners[0].as_ref().unwrap().split());

        assert!(combiners[0].as_mut().unwrap().advance());
        assert_eq!(&Split(&[1, 2, 3], 0), combiners[0].as_ref().unwrap().split());
        
        assert!(!combiners[0].as_mut().unwrap().advance());
    }
    
    #[test]
    fn safety_move_borrowed() {
        let mut combiners: [Option<SplitCombiner>; 2] = [const { None }; 2];
        
        let mut ordinals: Vec<usize> = vec![0; 3];
        combiners[0] = Some(SplitCombiner::new_no_alloc(CaptureMut::Borrowed(&mut ordinals)));
        assert_eq!(&Split(&[0, 1, 2], 3), combiners[0].as_ref().unwrap().split());

        combiners[1] = combiners[0].take();
        assert_eq!(&Split(&[0, 1, 2], 3), combiners[1].as_ref().unwrap().split());

        assert!(combiners[1].as_mut().unwrap().advance());
        assert_eq!(&Split(&[0, 1, 3], 2), combiners[1].as_ref().unwrap().split());

        assert!(combiners[1].as_mut().unwrap().advance());
        assert_eq!(&Split(&[0, 2, 3], 1), combiners[1].as_ref().unwrap().split());

        combiners[0] = combiners[1].take();
        assert_eq!(&Split(&[0, 2, 3], 1), combiners[0].as_ref().unwrap().split());

        assert!(combiners[0].as_mut().unwrap().advance());
        assert_eq!(&Split(&[1, 2, 3], 0), combiners[0].as_ref().unwrap().split());

        assert!(!combiners[0].as_mut().unwrap().advance());
    }
}