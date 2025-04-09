use crate::capture::CaptureMut;

#[derive(Debug)]
pub struct SplitCombiner<'a> {
    ordinals: CaptureMut<'a, Vec<usize>, [usize]>,
    omitted: usize,
}

#[derive(Debug)]
pub struct Partition<'a>(pub &'a [usize], pub usize);

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
        for ordinal in 0..ordinals.len() {
            ordinals[ordinal] = ordinal;
        }
        let ordinals_len = ordinals.len();
        Self {
            ordinals,
            omitted: ordinals_len,
        }
    }
    
    #[inline]
    pub fn split(&self) -> Partition {
        Partition(&self.ordinals, self.omitted)
    }

    #[inline]
    pub fn advance(&mut self) -> bool {
        if self.omitted != 0 {
            self.omitted -= 1;
            let mut index = 0;
            for ordinal in 0..self.ordinals.len() + 1 {
                if ordinal != self.omitted {
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
    use crate::comb::split_combiner::{Partition, SplitCombiner};

    fn collect_splits(mut splitter: SplitCombiner) -> Vec<(Vec<usize>, usize)> {
        let mut outputs = vec![];
        loop {
            let Partition(ordinals, omitted) = splitter.split();
            outputs.push((ordinals.to_owned(), omitted));
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
}