use crate::comb::combiner::Combiner;
use crate::comb::generator::Generator;
use crate::comb::split_combiner::{Split, SplitCombiner};

pub fn permute(n: usize, r: usize, mut f: impl FnMut(&[usize]) -> bool) {
    let mut combiner = Combiner::new(n, r); //TODO alloc
    let mut stack = vec![0; r];  //TODO alloc
    loop {
        //println!("combination: {:?}", combiner.ordinals());
        let elements = combiner.ordinals().iter().copied().collect::<Vec<_>>(); //TODO alloc
        if !_permute(&elements, &mut stack, &mut f, 0) {
            break;
        }
        if !combiner.advance() {
            break;
        }
    }
}

fn _permute(elements: &[usize], stack: &mut [usize], f: &mut impl FnMut(&[usize]) -> bool, depth: usize) -> bool {
    if !elements.is_empty() {
        let mut splitter = SplitCombiner::new(elements.len());  //TODO alloc
        loop {
            let Split(head, tail) = splitter.split();
            let head_ordinals = head.iter().map(|head_ordinal| elements[*head_ordinal]).collect::<Vec<_>>();  //TODO alloc
            let tail_ordinal = elements[tail];
            println!("{}permuting split {head_ordinals:?}-{tail_ordinal}, stack: {:?}", "  ".repeat(depth), &stack[stack.len() - depth..]);
            stack[stack.len() - depth - 1] = tail_ordinal;
            
            if !_permute(&head_ordinals, stack, f, depth + 1) {
                return false;
            }
            
            if !splitter.advance() {
                break;
            }
        }
        true
    } else {
        println!("{} feeding stack: {stack:?}", "  ".repeat(depth));
        // let mut inv_stack = stack.to_owned();
        // inv_stack.reverse();
        f(&stack)
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::sticky::{permute};
    use crate::comb::tests::inner_array_to_vec;

    fn iterate_sticky(n: usize, r: usize) -> Vec<Vec<usize>> {
        let mut outputs = vec![];
        permute(n, r, |ordinals| {
            let ordinals = ordinals.iter().map(|ordinal| *ordinal).collect::<Vec<_>>();
            outputs.push(ordinals);
            true
        });
        outputs
    }

    #[test]
    fn permute_0p0() {
        let outputs = iterate_sticky(0, 0);
        let expected_outputs = vec![
            []
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permute_1p0() {
        let outputs = iterate_sticky(1, 0);
        let expected_outputs = vec![
            []
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permute_1p1() {
        let outputs = iterate_sticky(1, 1);
        let expected_outputs = vec![
            [0]
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permute_4p0() {
        let outputs = iterate_sticky(4, 0);
        let expected_outputs = vec![
            []
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permute_4p2() {
        let outputs = iterate_sticky(4, 2);
        let expected_outputs = vec![
            [0, 1],
            [1, 0],
            [0, 2],
            [2, 0],
            [0, 3],
            [3, 0],
            [1, 2],
            [2, 1],
            [1, 3],
            [3, 1],
            [2, 3],
            [3, 2]
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permute_4p3() {
        let outputs = iterate_sticky(4, 3);
        let expected_outputs = vec![
            [0, 1, 2],
            [1, 0, 2],
            [0, 2, 1],
            [2, 0, 1],
            [1, 2, 0],
            [2, 1, 0],
            [0, 1, 3],
            [1, 0, 3],
            [0, 3, 1],
            [3, 0, 1],
            [1, 3, 0],
            [3, 1, 0],
            [0, 2, 3],
            [2, 0, 3],
            [0, 3, 2],
            [3, 0, 2],
            [2, 3, 0],
            [3, 2, 0],
            [1, 2, 3],
            [2, 1, 3],
            [1, 3, 2],
            [3, 1, 2],
            [2, 3, 1],
            [3, 2, 1]
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permute_4p4() {
        let outputs = iterate_sticky(4, 4);
        let expected_outputs = vec![
            [0, 1, 2, 3],
            [1, 0, 2, 3],
            [0, 2, 1, 3],
            [2, 0, 1, 3],
            [1, 2, 0, 3],
            [2, 1, 0, 3],
            [0, 1, 3, 2],
            [1, 0, 3, 2],
            [0, 3, 1, 2],
            [3, 0, 1, 2],
            [1, 3, 0, 2],
            [3, 1, 0, 2],
            [0, 2, 3, 1],
            [2, 0, 3, 1],
            [0, 3, 2, 1],
            [3, 0, 2, 1],
            [2, 3, 0, 1],
            [3, 2, 0, 1],
            [1, 2, 3, 0],
            [2, 1, 3, 0],
            [1, 3, 2, 0],
            [3, 1, 2, 0],
            [2, 3, 1, 0],
            [3, 2, 1, 0]
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permute_4p2_subset() {
        const CAP: usize = 6;
        let mut outputs = vec![];
        let mut permutation = 0;
        permute(4, 2, |ordinals| {
            let ordinals = ordinals.iter().map(|ordinal| *ordinal).collect::<Vec<_>>();
            outputs.push(ordinals);
            permutation += 1;
            permutation < CAP
        });
        let expected_outputs = vec![
            [0, 1],
            [1, 0],
            [0, 2],
            [2, 0],
            [0, 3],
            [3, 0],
        ];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    // #[test]
    // fn splitter() {
    //     const LEN: usize = 16;
    //     let all = Bitmap::from(([0, 5, 10, 15], LEN));
    //     let splits = Splitter::new(&all).collect::<Vec<_>>();
    //     println!("splits: {splits:?}");
    //     let expected = vec![
    //         Split(Bitmap::from(([0, 5, 10], LEN)), 15), 
    //         Split(Bitmap::from(([0, 5, 15], LEN)), 10),
    //         Split(Bitmap::from(([0, 10, 15], LEN)), 5),
    //         Split(Bitmap::from(([5, 10, 15], LEN)), 0),
    //     ];
    //     assert_eq!(expected, splits);
    // }
}
