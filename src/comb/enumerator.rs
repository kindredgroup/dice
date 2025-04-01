use crate::comb::{count_states, pick_state};

pub struct Enumerator<'a> {
    cardinalities: &'a [usize],
    states: u64,
}
impl<'a> Enumerator<'a> {
    pub fn new(cardinalities: &'a [usize]) -> Self {
        let states = count_states(cardinalities);
        Self {
            cardinalities,
            states,
        }
    }
}

impl<'a> IntoIterator for Enumerator<'a> {
    type Item = Vec<usize>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            enumerator: self,
            index: 0,
        }
    }
}

pub struct Iter<'a> {
    enumerator: Enumerator<'a>,
    index: u64,
}
impl Iterator for Iter<'_> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index != self.enumerator.states {
            let mut ordinals = vec![0; self.enumerator.cardinalities.len()];
            pick_state(self.enumerator.cardinalities, self.index, &mut ordinals);
            self.index += 1;
            Some(ordinals)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::enumerator::Enumerator;
    use crate::comb::tests::inner_array_to_vec;

    #[test]
    fn iterator() {
        let enumerator = Enumerator::new(&[2, 3, 4]);
        let outputs = enumerator.into_iter().collect::<Vec<_>>();
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