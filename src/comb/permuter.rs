use crate::comb::{is_full, take_next_available};

#[derive(Debug)]
pub struct Permuter<'a> {
    r: usize,
    bitmap: &'a mut [bool],
    ordinals: &'a mut [usize],
}

impl<'a> Permuter<'a> {
    #[inline]
    pub fn new_no_alloc(r: usize, bitmap: &'a mut [bool], ordinals: &'a mut [usize]) -> Self {
        for (index, ordinal) in ordinals.iter_mut().enumerate() {
            *ordinal = index;
            bitmap[*ordinal] = true;
        }
        for i in ordinals.len()..bitmap.len() {
            bitmap[i] = false;
        }

        Self {
            r,
            bitmap,
            ordinals,
        }
    }

    #[inline]
    pub fn ordinals(&self) -> &[usize] {
        &self.ordinals
    }

    #[inline]
    pub fn step(&mut self) -> bool {
        if self.r == 0 {
            return false;
        }

        let mut caret = self.r - 1;
        loop {
            let ordinal = self.ordinals[caret];
            if is_full(&self.bitmap, ordinal + 1) {
                //println!("full at caret {caret}");
                self.bitmap[ordinal] = false;
                if caret != 0 {
                    caret -= 1;
                } else {
                    return false;
                }
            } else {
                break;
            }
        }

        //println!("caret={caret}, bitmap={:?}", self.bitmap);
        let next_available =
            take_next_available(&mut self.bitmap, self.ordinals[caret] + 1).unwrap();
        self.bitmap[self.ordinals[caret]] = false;
        self.ordinals[caret] = next_available;

        for index in caret + 1..self.r {
            //println!("  index={index}, bitmap={:?}", self.bitmap);
            self.ordinals[index] = take_next_available(&mut self.bitmap, 0).unwrap()
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::permuter::Permuter;
    use crate::comb::tests::inner_array_to_vec;

    fn iterate_permuter(n: usize, r: usize) -> Vec<Vec<usize>> {
        let mut bitmap = vec![false; n];
        let mut ordinals = vec![0; r];
        let mut permuter = Permuter::new_no_alloc(r, &mut bitmap, &mut ordinals);
        println!("ordinals:");
        let mut outputs = Vec::new();
        loop {
            let ordinals = permuter
                .ordinals()
                .iter()
                .map(|&ordinal| ordinal)
                .collect::<Vec<_>>();
            println!("{ordinals:?},");
            outputs.push(ordinals);
            if !permuter.step() {
                break;
            }
        }
        outputs
    }

    #[test]
    fn permuter_4p0() {
        let outputs = iterate_permuter(4, 0);
        let expected_outputs = vec![
            []
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permuter_4p1() {
        let outputs = iterate_permuter(4, 1);
        let expected_outputs = vec![
            [0],
            [1],
            [2],
            [3],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permuter_4p2() {
        let outputs = iterate_permuter(4, 2);
        let expected_outputs = vec![
            [0, 1],
            [0, 2],
            [0, 3],
            [1, 0],
            [1, 2],
            [1, 3],
            [2, 0],
            [2, 1],
            [2, 3],
            [3, 0],
            [3, 1],
            [3, 2],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permuter_4p3() {
        let outputs = iterate_permuter(4, 3);
        let expected_outputs = vec![
            [0, 1, 2],
            [0, 1, 3],
            [0, 2, 1],
            [0, 2, 3],
            [0, 3, 1],
            [0, 3, 2],
            [1, 0, 2],
            [1, 0, 3],
            [1, 2, 0],
            [1, 2, 3],
            [1, 3, 0],
            [1, 3, 2],
            [2, 0, 1],
            [2, 0, 3],
            [2, 1, 0],
            [2, 1, 3],
            [2, 3, 0],
            [2, 3, 1],
            [3, 0, 1],
            [3, 0, 2],
            [3, 1, 0],
            [3, 1, 2],
            [3, 2, 0],
            [3, 2, 1],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permuter_4p4() {
        let outputs = iterate_permuter(4, 4);
        let expected_outputs = vec![
            [0, 1, 2, 3],
            [0, 1, 3, 2],
            [0, 2, 1, 3],
            [0, 2, 3, 1],
            [0, 3, 1, 2],
            [0, 3, 2, 1],
            [1, 0, 2, 3],
            [1, 0, 3, 2],
            [1, 2, 0, 3],
            [1, 2, 3, 0],
            [1, 3, 0, 2],
            [1, 3, 2, 0],
            [2, 0, 1, 3],
            [2, 0, 3, 1],
            [2, 1, 0, 3],
            [2, 1, 3, 0],
            [2, 3, 0, 1],
            [2, 3, 1, 0],
            [3, 0, 1, 2],
            [3, 0, 2, 1],
            [3, 1, 0, 2],
            [3, 1, 2, 0],
            [3, 2, 0, 1],
            [3, 2, 1, 0],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }
}