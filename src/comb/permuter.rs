use crate::capture::CaptureMut;
use crate::comb::{is_full, take_next_available};
use crate::comb::generator::Generator;

#[derive(Debug)]
pub struct Alloc<'a> {
    pub bitmap: CaptureMut<'a, Vec<bool>, [bool]>,
    pub ordinals: CaptureMut<'a, Vec<usize>, [usize]>,
}

impl Alloc<'_> {
    #[inline]
    pub fn new(n: usize, r: usize) -> Self {
        Self {
            bitmap: vec![false; n].into(),
            ordinals: vec![0; r].into(),
        }
    }
}

#[derive(Debug)]
pub struct Permuter<'a> {
    r: usize,
    bitmap: CaptureMut<'a, Vec<bool>, [bool]>,
    ordinals: CaptureMut<'a, Vec<usize>, [usize]>,
}

impl<'a> Permuter<'a> {
    #[inline]
    pub fn new(n: usize, r: usize) -> Self {
        Self::new_no_alloc(r, Alloc::new(n, r))
    }
    
    #[inline]
    pub fn new_no_alloc(r: usize, alloc: Alloc<'a>) -> Self {
        let Alloc { mut bitmap, mut ordinals } = alloc;
        debug_assert_eq!(r, ordinals.len(), "length of ordinals must equal r");
        debug_assert!(bitmap.len() >= r, "length of bitmap must be greater or equal to r");
        
        for (index, ordinal) in ordinals.iter_mut().enumerate() {
            *ordinal = index;
            bitmap[*ordinal] = true;
        }
        for i in ordinals.len()..bitmap.len() {
            bitmap[i] = false;
        }
        for ordinal in 0..ordinals.len() {
            ordinals[ordinal] = ordinal;
        }

        Self {
            r,
            bitmap,
            ordinals,
        }
    }
}

impl Generator for Permuter<'_> {
    #[inline]
    fn ordinals(&self) -> &[usize] {
        &self.ordinals
    }

    #[inline]
    fn advance(&mut self) -> bool {
        if self.r == 0 {
            return false;
        }

        let mut caret = self.r - 1;
        loop {
            let ordinal = self.ordinals[caret];
            if is_full(&self.bitmap, ordinal + 1) {
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

        let next_available =
            take_next_available(&mut self.bitmap, self.ordinals[caret] + 1).unwrap();
        self.bitmap[self.ordinals[caret]] = false;
        self.ordinals[caret] = next_available;

        for index in caret + 1..self.r {
            self.ordinals[index] = take_next_available(&mut self.bitmap, 0).unwrap()
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::permuter::Permuter;
    use crate::comb::tests::{inner_array_to_vec, iterate_generator};

    #[test]
    fn permuter_0p0() {
        let outputs = iterate_generator(Permuter::new(0, 0));
        let expected_outputs = vec![
            []
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permuter_1p0() {
        let outputs = iterate_generator(Permuter::new(1, 0));
        let expected_outputs = vec![
            []
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permuter_1p1() {
        let outputs = iterate_generator(Permuter::new(1, 1));
        let expected_outputs = vec![
            [0]
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permuter_4p0() {
        let outputs = iterate_generator(Permuter::new(4, 0));
        let expected_outputs = vec![
            []
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permuter_4p1() {
        let outputs = iterate_generator(Permuter::new(4, 1));
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
        let outputs = iterate_generator(Permuter::new(4, 2));
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
        let outputs = iterate_generator(Permuter::new(4, 3));
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
        let outputs = iterate_generator(Permuter::new(4, 4));
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