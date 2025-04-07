//! Combinatorics.

pub mod bitmap;
pub mod combiner;
pub mod enumerator;
pub mod generator;
pub mod permuter;
pub mod sticky;

#[inline]
pub fn pick_state(cardinalities: &[usize], state_index: usize, ordinals: &mut [usize]) {
    let mut residual = state_index;
    for (index, &cardinality) in cardinalities.iter().enumerate() {
        let cardinality = cardinality;
        let (quotient, remainder) = (residual / cardinality, residual % cardinality);
        residual = quotient;
        ordinals[index] = remainder;
    }
}

#[inline]
pub fn pick_state_hyper(
    cardinality: usize,
    dimensions: usize,
    state_index: usize,
    ordinals: &mut [usize],
) {
    let mut residual = state_index;
    for index in 0..dimensions {
        let (quotient, remainder) = (residual / cardinality, residual % cardinality);
        residual = quotient;
        ordinals[index] = remainder;
    }
}

#[inline]
pub fn count_states(cardinalities: &[usize]) -> usize {
    cardinalities
        .iter()
        .fold(1, |acc, &num| acc * num)
}

#[inline(always)]
pub fn is_unique_quadratic(elements: &[usize]) -> bool {
    for (index, element) in elements.iter().enumerate() {
        for other in &elements[index + 1..] {
            if element == other {
                return false;
            }
        }
    }
    true
}

#[inline(always)]
pub fn is_unique_linear(elements: &[usize], bitmap: &mut [bool]) -> bool {
    bitmap.fill(false);
    for &element in elements {
        if bitmap[element] {
            return false;
        }
        bitmap[element] = true;
    }
    true
}

#[inline]
pub fn count_permutations(n: usize, r: usize) -> usize {
    ((n - r + 1)..=n).product()
}

#[inline]
pub fn pick_permutation(
    cardinality: usize,
    permutation_index: usize,
    bitmap: &mut [bool],
    ordinals: &mut [usize],
) {
    bitmap.fill(false);
    let mut residual = permutation_index;
    // let mut lowest = ordinals.len();
    for index in 0..ordinals.len() {
        let cardinality = cardinality - index;
        let (quotient, remainder) = (residual / cardinality, residual % cardinality);
        residual = quotient;
        // if remainder < lowest {
        if index == 0 {
            ordinals[index] = remainder;
            bitmap[remainder] = true;
            // if remainder < lowest {
            //     lowest = remainder;
            // }
        } else {
            let mut free = 0;
            for b in 0..bitmap.len() {
                if bitmap[b] {
                    continue;
                }
                if free == remainder {
                    ordinals[index] = b;
                    bitmap[b] = true;
                    // if b < lowest {
                    //     lowest = b;
                    // }
                    break;
                } else {
                    free += 1;
                }
            }
        }
    }
}

#[inline]
pub fn pick_permutation_reverse(
    cardinality: usize,
    permutation_index: usize,
    bitmap: &mut [bool],
    ordinals: &mut [usize],
) {
    bitmap.fill(false);
    let mut residual = permutation_index;
    // let mut lowest = ordinals.len();
    let lim = ordinals.len();
    for index in 0..lim {
        let cardinality = cardinality - index;
        let (quotient, remainder) = (residual / cardinality, residual % cardinality);
        residual = quotient;
        // if remainder < lowest {
        if index == 0 {
            ordinals[lim - index - 1] = remainder;
            bitmap[remainder] = true;
            // if remainder < lowest {
            //     lowest = remainder;
            // }
        } else {
            let mut free = 0;
            for b in 0..bitmap.len() {
                if bitmap[b] {
                    continue;
                }
                if free == remainder {
                    ordinals[lim - index - 1] = b;
                    bitmap[b] = true;
                    // if b < lowest {
                    //     lowest = b;
                    // }
                    break;
                } else {
                    free += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::generator::Generator;
    use crate::itemiser::Itemiser;
    use super::*;

    pub(crate) fn iterate_generator(generator: impl Generator) -> Vec<Vec<usize>> {
        let outputs = generator.into_itemiser().into_vec();
        println!("ordinals:");
        for ordinals in &outputs {
            println!("{ordinals:?}")
        }
        outputs
    }

    #[test]
    fn test_pick_state() {
        let cardinalities = &[2, 3, 4];
        let mut outputs = vec![];
        let states = count_states(cardinalities);
        assert_eq!(24, states);
        for index in 0..states {
            let mut ordinals = [0; 3];
            pick_state(cardinalities, index, &mut ordinals);
            outputs.push(ordinals.to_vec());
            println!("ordinals: {ordinals:?}");
        }
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

    #[test]
    fn test_is_unique_quadratic() {
        assert!(is_unique_quadratic(&[]));
        assert!(is_unique_quadratic(&[1]));
        assert!(is_unique_quadratic(&[1, 2, 3]));
        assert!(!is_unique_quadratic(&[1, 1]));
        assert!(!is_unique_quadratic(&[1, 0, 1]));
    }

    #[test]
    fn test_is_unique_linear() {
        let mut bitmap_0 = vec![false; 0];
        let mut bitmap_1 = vec![false; 1];
        let mut bitmap_2 = vec![false; 2];
        let mut bitmap_3 = vec![false; 3];

        assert!(is_unique_linear(&[], &mut bitmap_0));
        assert!(is_unique_linear(&[0], &mut bitmap_1));
        assert!(is_unique_linear(&[0, 1, 2], &mut bitmap_3));
        assert!(is_unique_linear(&[2, 1, 0], &mut bitmap_3));
        assert!(!is_unique_linear(&[0, 0], &mut bitmap_2));
        assert!(!is_unique_linear(&[1, 0, 1], &mut bitmap_3));
    }

    #[test]
    fn test_count_permutations() {
        assert_eq!(1, count_permutations(0, 0));
        assert_eq!(1, count_permutations(1, 0));
        assert_eq!(1, count_permutations(1, 1));
        assert_eq!(1, count_permutations(2, 0));
        assert_eq!(2, count_permutations(2, 1));
        assert_eq!(2, count_permutations(2, 2));
        assert_eq!(1, count_permutations(3, 0));
        assert_eq!(3, count_permutations(3, 1));
        assert_eq!(6, count_permutations(3, 2));
        assert_eq!(6, count_permutations(3, 3));
        assert_eq!(1, count_permutations(4, 0));
        assert_eq!(4, count_permutations(4, 1));
        assert_eq!(12, count_permutations(4, 2));
        assert_eq!(24, count_permutations(4, 3));
        assert_eq!(24, count_permutations(4, 4));
    }

    fn generate_permutations(n: usize, r: usize) -> Vec<Vec<usize>> {
        let mut outputs = vec![];
        let permutations = count_permutations(n, r);
        let mut bitmap = vec![false; n];
        println!("ordinals:");
        for index in 0..permutations {
            let mut ordinals = vec![0; r];
            pick_permutation(n, index, &mut bitmap, &mut ordinals);
            outputs.push(ordinals.to_vec());
            println!("{ordinals:?},");
        }
        outputs
    }

    fn generate_permutations_reverse(n: usize, r: usize) -> Vec<Vec<usize>> {
        let mut outputs = vec![];
        let permutations = count_permutations(n, r);
        let mut bitmap = vec![false; n];
        println!("ordinals:");
        for index in (0..permutations).into_iter().rev() {
            let mut ordinals = vec![0; r];
            pick_permutation_reverse(n, index, &mut bitmap, &mut ordinals);
            outputs.push(ordinals.to_vec());
            println!("{ordinals:?},");
        }
        outputs
    }

    pub(crate) fn inner_array_to_vec<const N: usize>(input: Vec<[usize; N]>) -> Vec<Vec<usize>> {
        input.iter().map(|array| array.to_vec()).collect()
    }

    #[test]
    fn test_pick_permutation_3p3() {
        let outputs = generate_permutations(3, 3);
        let expected_outputs = vec![
            [0, 1, 2],
            [1, 0, 2],
            [2, 0, 1],
            [0, 2, 1],
            [1, 2, 0],
            [2, 1, 0],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn test_pick_permutation_4p4() {
        let outputs = generate_permutations(4, 4);
        let expected_outputs = vec![
            [0, 1, 2, 3],
            [1, 0, 2, 3],
            [2, 0, 1, 3],
            [3, 0, 1, 2],
            [0, 2, 1, 3],
            [1, 2, 0, 3],
            [2, 1, 0, 3],
            [3, 1, 0, 2],
            [0, 3, 1, 2],
            [1, 3, 0, 2],
            [2, 3, 0, 1],
            [3, 2, 0, 1],
            [0, 1, 3, 2],
            [1, 0, 3, 2],
            [2, 0, 3, 1],
            [3, 0, 2, 1],
            [0, 2, 3, 1],
            [1, 2, 3, 0],
            [2, 1, 3, 0],
            [3, 1, 2, 0],
            [0, 3, 2, 1],
            [1, 3, 2, 0],
            [2, 3, 1, 0],
            [3, 2, 1, 0],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn test_pick_permutation_4p2() {
        let outputs = generate_permutations(4, 2);
        let expected_outputs = vec![
            [0, 1],
            [1, 0],
            [2, 0],
            [3, 0],
            [0, 2],
            [1, 2],
            [2, 1],
            [3, 1],
            [0, 3],
            [1, 3],
            [2, 3],
            [3, 2],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn test_pick_permutation_4p1() {
        let outputs = generate_permutations(4, 1);
        let expected_outputs = vec![[0], [1], [2], [3]];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn test_pick_permutation_4p0() {
        let outputs = generate_permutations(4, 0);
        let expected_outputs = vec![[]];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn test_pick_permutation_reverse_3p3() {
        let outputs = generate_permutations_reverse(3, 3);
        let expected_outputs = vec![[0, 1, 2], [0, 2, 1], [1, 2, 0], [1, 0, 2], [2, 0, 1], [
            2, 1, 0,
        ]];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn test_pick_permutation_reverse_4p4() {
        let outputs = generate_permutations_reverse(4, 4);
        let expected_outputs = vec![
            [0, 1, 2, 3],
            [0, 1, 3, 2],
            [0, 2, 3, 1],
            [1, 2, 3, 0],
            [0, 2, 1, 3],
            [0, 3, 1, 2],
            [0, 3, 2, 1],
            [1, 3, 2, 0],
            [1, 2, 0, 3],
            [1, 3, 0, 2],
            [2, 3, 0, 1],
            [2, 3, 1, 0],
            [1, 0, 2, 3],
            [1, 0, 3, 2],
            [2, 0, 3, 1],
            [2, 1, 3, 0],
            [2, 0, 1, 3],
            [3, 0, 1, 2],
            [3, 0, 2, 1],
            [3, 1, 2, 0],
            [2, 1, 0, 3],
            [3, 1, 0, 2],
            [3, 2, 0, 1],
            [3, 2, 1, 0],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    // #[test]
    // fn test_pick_permutation_reverse_4p2() {
    //     let outputs = generate_permutations_reverse(4, 2);
    //     let expected_outputs = vec![
    //         [0, 1],
    //         [1, 0],
    //         [2, 0],
    //         [3, 0],
    //         [0, 2],
    //         [1, 2],
    //         [2, 1],
    //         [3, 1],
    //         [0, 3],
    //         [1, 3],
    //         [2, 3],
    //         [3, 2],
    //     ];
    //     assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    // }
}
