use crate::capture::CaptureMut;
use crate::comb::generator::Generator;

#[derive(Debug)]
pub struct Combiner<'a> {
    ordinals: CaptureMut<'a, Vec<usize>, [usize]>,
    n: usize,
}

impl<'a> Combiner<'a> {
    #[inline]
    pub fn new(n: usize, r: usize) -> Self {
        Self::new_no_alloc(n, vec![0; r].into())
    }

    #[inline]
    pub fn new_no_alloc(n: usize, mut ordinals: CaptureMut<'a, Vec<usize>, [usize]>) -> Self {
        for ordinal in 0..ordinals.len() {
            ordinals[ordinal] = ordinal;
        }
        Self {
            ordinals, n,
        }
    }
}

impl Generator for Combiner<'_> {
    #[inline]
    fn ordinals(&self) -> &[usize] {
        &self.ordinals
    }

    #[inline]
    fn advance(&mut self) -> bool {
        if self.ordinals.is_empty() {
            return false;
        }

        let mut caret = self.ordinals.len() - 1;
        loop {
            let lim = if caret == self.ordinals.len() - 1 { self.n - 1 } else { self.ordinals[caret + 1] - 1 };
            //println!("ordinals: {:?}, caret: {caret}, lim: {lim}", self.ordinals);
            if self.ordinals[caret] < lim {
                break;
            }
            if caret != 0 {
                caret -= 1;
            } else {
                return false;
            }
        }

        self.ordinals[caret] += 1;
        for i in caret + 1..self.ordinals.len() {
            self.ordinals[i] = self.ordinals[i - 1] + 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::combiner::Combiner;
    use crate::comb::generator::Generator;
    use crate::comb::tests::inner_array_to_vec;

    fn iterate_combiner(n: usize, r: usize) -> Vec<Vec<usize>> {
        let mut combiner = Combiner::new(n, r);
        println!("ordinals:");
        let mut outputs = Vec::new();
        loop {
            let ordinals = combiner
                .ordinals()
                .iter()
                .map(|&ordinal| ordinal)
                .collect::<Vec<_>>();
            println!("{ordinals:?},");
            outputs.push(ordinals);
            if !combiner.advance() {
                break;
            }
        }
        outputs
    }

    #[test]
    fn combiner_0c0() {
        let outputs = iterate_combiner(0, 0);
        let expected_outputs = vec![
            []
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn combiner_1c0() {
        let outputs = iterate_combiner(1, 0);
        let expected_outputs = vec![
            []
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn combiner_1c1() {
        let outputs = iterate_combiner(1, 1);
        let expected_outputs = vec![
            [0]
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn combiner_2c1() {
        let outputs = iterate_combiner(2, 1);
        let expected_outputs = vec![
            [0],
            [1]
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn combiner_2c2() {
        let outputs = iterate_combiner(2, 2);
        let expected_outputs = vec![
            [0, 1],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn combiner_3c0() {
        let outputs = iterate_combiner(3, 0);
        let expected_outputs = vec![
            []
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn combiner_3c1() {
        let outputs = iterate_combiner(3, 1);
        let expected_outputs = vec![
            [0],
            [1],
            [2]
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn combiner_3c2() {
        let outputs = iterate_combiner(3, 2);
        let expected_outputs = vec![
            [0, 1],
            [0, 2],
            [1, 2]
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn combiner_3c3() {
        let outputs = iterate_combiner(3, 3);
        let expected_outputs = vec![
            [0, 1, 2],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn combiner_6c4() {
        let outputs = iterate_combiner(6, 4);
        let expected_outputs = vec![
            [0, 1, 2, 3],
            [0, 1, 2, 4],
            [0, 1, 2, 5],
            [0, 1, 3, 4],
            [0, 1, 3, 5],
            [0, 1, 4, 5],
            [0, 2, 3, 4],
            [0, 2, 3, 5],
            [0, 2, 4, 5],
            [0, 3, 4, 5],
            [1, 2, 3, 4],
            [1, 2, 3, 5],
            [1, 2, 4, 5],
            [1, 3, 4, 5],
            [2, 3, 4, 5],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }
}