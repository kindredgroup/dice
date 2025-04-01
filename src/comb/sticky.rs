use crate::comb::bitmap::Bitmap;
use crate::comb::combiner::Combiner;

pub fn permute(n: usize, r: usize, mut f: impl FnMut(&[usize])) {
    let mut combiner = Combiner::new(n, r);
    loop {
        println!("combination: {:?}", combiner.ordinals());
        let elements = Bitmap::from((combiner.ordinals().iter().map(|ordinal| *ordinal), n));
        let stack = vec![];
        _permute(&elements, &stack, &mut f, 0);
        if !combiner.step() {
            break;
        }
    }
}

fn _permute(elements: &Bitmap, stack: &[usize], f: &mut impl FnMut(&[usize]), depth: usize) {
    if !elements.is_empty() {
        for Split(head, tail) in Splitter::new(&elements) {
            println!("{}permuting split {head}-{tail}, stack: {stack:?}", "  ".repeat(depth));
            // let mut permutation = Vec::with_capacity(head.size() + 1 + stack.len());
            // for ordinal in head.ordinals() {
            //     permutation.push(ordinal);
            // }
            // permutation.push(tail);
            // for &ordinal in stack {
            //     permutation.push(ordinal);
            // }
            
            let mut new_stack = Vec::with_capacity(stack.len() + 1);
            new_stack.push(tail);
            for ordinal in stack {
                new_stack.push(*ordinal);
            }
            _permute(&head, &new_stack, f, depth + 1)
        }
    } else {
        f(stack)
    }
}

struct Splitter<'a> {
    all: &'a Bitmap,
    ordinal: Option<usize>,
}

impl<'a> Splitter<'a> {
    pub fn new(all: &'a Bitmap) -> Self {
        Self {
            all,
            ordinal: Some(all.len() - 1)
        }
    }
}

impl<'a> Iterator for Splitter<'a> {
    type Item = Split;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.ordinal {
                None => return None,
                Some(ordinal) => {
                    let mut subset = self.all.clone();
                    let prev = *ordinal;
                    if *ordinal != 0 {
                        *ordinal -= 1;
                    } else {
                        self.ordinal = None
                    }
                    if subset[prev] {
                        subset[prev] = false;
                        return Some(Split(subset, prev))
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Split(Bitmap, usize);

#[cfg(test)]
mod tests {
    use crate::comb::bitmap::Bitmap;
    use crate::comb::sticky::{permute, Split, Splitter};
    use crate::comb::tests::inner_array_to_vec;

    fn iterate_sticky(n: usize, r: usize) -> Vec<Vec<usize>> {
        let mut outputs = vec![];
        permute(n, r, |ordinals| {
            let ordinals = ordinals.iter().map(|ordinal| *ordinal).collect::<Vec<_>>();
            outputs.push(ordinals);
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
    fn splitter() {
        const LEN: usize = 16;
        let all = Bitmap::from(([0, 5, 10, 15], LEN));
        let splits = Splitter::new(&all).collect::<Vec<_>>();
        println!("splits: {splits:?}");
        let expected = vec![
            Split(Bitmap::from(([0, 5, 10], LEN)), 15), 
            Split(Bitmap::from(([0, 5, 15], LEN)), 10),
            Split(Bitmap::from(([0, 10, 15], LEN)), 5),
            Split(Bitmap::from(([5, 10, 15], LEN)), 0),
        ];
        assert_eq!(expected, splits);
    }
}
