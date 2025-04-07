use crate::capture::CaptureMut;
use crate::comb::{count_states, pick_state};
use crate::comb::itemiser::Itemiser;

pub struct Enumerator<'a> {
    cardinalities: &'a [usize],
    ordinals: CaptureMut<'a, Vec<usize>, [usize]>,
    index: usize,
    states: usize,
}

impl<'a> Enumerator<'a> {
    #[inline]
    pub fn new(cardinalities: &'a [usize]) -> Self {
        Self::new_no_alloc(cardinalities, vec![0; cardinalities.len()].into())
    }
    
    #[inline]
    pub fn new_no_alloc(cardinalities: &'a [usize], ordinals: CaptureMut<'a, Vec<usize>, [usize]>) -> Self {
        debug_assert_eq!(cardinalities.len(), ordinals.len(), "length of cardinalities must equal to length of ordinals");
        
        let states = count_states(cardinalities);
        Self {
            cardinalities,
            ordinals,
            index: 0,
            states,
        }
    }
}

impl Itemiser for Enumerator<'_> {
    type Item = [usize];

    fn next(&mut self) -> Option<&[usize]> {
        if self.index != self.states {
            pick_state(self.cardinalities, self.index, &mut self.ordinals);
            self.index += 1;
            Some(&self.ordinals)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::enumerator::Enumerator;
    use crate::comb::itemiser::Itemiser;
    use crate::comb::tests::inner_array_to_vec;

    #[test]
    fn iterator_0() {
        let enumerator = Enumerator::new(&[]);
        let outputs = enumerator.collect();
        let expected_outputs = vec![
            [],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn iterator_1() {
        let enumerator = Enumerator::new(&[1]);
        let outputs = enumerator.collect();
        let expected_outputs = vec![
            [0]
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn iterator_1_empty() {
        let enumerator = Enumerator::new(&[0]);
        let outputs = enumerator.collect();
        let expected_outputs: Vec<[usize; 0]> = vec![
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn iterator_3() {
        let enumerator = Enumerator::new(&[2, 3, 4]);
        let outputs = enumerator.collect();
        let expected_outputs = vec![
            [0, 0, 0],
            [1, 0, 0],
            [0, 1, 0],
            [1, 1, 0],
            [0, 2, 0],
            [1, 2, 0],
            [0, 0, 1],
            [1, 0, 1],
            [0, 1, 1],
            [1, 1, 1],
            [0, 2, 1],
            [1, 2, 1],
            [0, 0, 2],
            [1, 0, 2],
            [0, 1, 2],
            [1, 1, 2],
            [0, 2, 2],
            [1, 2, 2],
            [0, 0, 3],
            [1, 0, 3],
            [0, 1, 3],
            [1, 1, 3],
            [0, 2, 3],
            [1, 2, 3],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }
}