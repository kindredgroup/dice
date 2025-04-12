//! A sticky, recursive permuter that prioritises permutations with the highest ordinals
//! in the least significant positions.
//! 
//! For example, in a <sup>4</sup>P<sub>4</sub> traversal, the initial permutation 
//! `[0, 1, 2, 3]` will be succeeded by `[1, 0, 2, 3]` and `[0, 2, 1, 3]`, keeping the
//! highest ordinals anchored for as long as possible.
//! 
//! This traversal strategy inhibits the admission of higher ordinals into the most significant 
//! positions for as long as possible.

use crate::capture::CaptureMut;
use crate::comb::combiner::Combiner;
use crate::comb::generator::Generator;
use crate::comb::split_combiner::{Split, SplitCombiner};

#[derive(Debug)]
pub struct Alloc<'a> {
    pub combiner_ordinals: CaptureMut<'a, Vec<usize>, [usize]>,
    pub whole_ordinals_stack: CaptureMut<'a, Vec<usize>, [usize]>,
    pub split_ordinals_stack: CaptureMut<'a, Vec<usize>, [usize]>,
    pub ordinals: CaptureMut<'a, Vec<usize>, [usize]>,
}

impl Alloc<'_> {
    #[inline]
    pub fn new(r: usize) -> Self {
        let stack_len = Self::stack_len(r);
        Self {
            combiner_ordinals: vec![0; r].into(),
            whole_ordinals_stack: vec![0; stack_len].into(),
            split_ordinals_stack: vec![0; stack_len].into(),
            ordinals: vec![0; r].into(),
        }
    }
    
    #[inline]
    pub fn stack_len(r: usize) -> usize {
        (r + 1) * r / 2
    }

    /// Takes sub-slices of the constituents to suit the needs of permuting
    /// over a smaller `r` than what was specified in the initial allocation,
    /// returning a borrowed projection of [`Self`].
    /// 
    /// This is used to avoid repeat allocations when permuting over varying
    /// values of `r`. Instead of allocating each time, allocate once
    /// for the largest expected value of `r` and later borrow slices for
    /// smaller `r` values.
    #[inline]
    pub fn shrink<'a: 'b, 'b>(&'a mut self, smaller_r: usize) -> Alloc<'b> {
        let stack_len = Self::stack_len(smaller_r);
        Alloc {
            combiner_ordinals: CaptureMut::Borrowed(
                &mut self.combiner_ordinals[..smaller_r],
            ),
            whole_ordinals_stack: CaptureMut::Borrowed(
                &mut self.whole_ordinals_stack[..stack_len],
            ),
            split_ordinals_stack: CaptureMut::Borrowed(
                &mut self.split_ordinals_stack[..stack_len],
            ),
            ordinals: CaptureMut::Borrowed(&mut self.ordinals[..smaller_r]),
        }
    }
}

#[inline]
pub fn permute(n: usize, r: usize, f: impl FnMut(&[usize]) -> bool) {
    permute_no_alloc(n, r, Alloc::new(r), f)
}

#[inline]
pub fn permute_no_alloc(n: usize, r: usize, alloc: Alloc, mut f: impl FnMut(&[usize]) -> bool) {
    let Alloc {
        combiner_ordinals,
        mut whole_ordinals_stack,
        mut split_ordinals_stack,
        mut ordinals,
    } = alloc;

    let expected_stack_len = Alloc::stack_len(r);
    debug_assert_eq!(
        combiner_ordinals.len(),
        r,
        "combiner ordinals length must equal r"
    );
    debug_assert_eq!(
        whole_ordinals_stack.len(),
        expected_stack_len,
        "whole ordinals stack length must equal r(r+1)/2"
    );
    debug_assert_eq!(
        split_ordinals_stack.len(),
        expected_stack_len,
        "split ordinals stack length must equal r(r+1)/2"
    );
    debug_assert_eq!(ordinals.len(), r, "ordinals length must equal r");

    let mut combiner = Combiner::new_no_alloc(n, combiner_ordinals);
    loop {
        //println!("combination: {:?}", combiner.ordinals());
        for (index, ordinal) in combiner.read().iter().enumerate() {
            whole_ordinals_stack[index] = *ordinal;
        }

        if !_permute_no_alloc(
            &mut whole_ordinals_stack,
            &mut split_ordinals_stack,
            0,
            r,
            &mut ordinals,
            &mut f,
            0,
        ) {
            break;
        }
        if !combiner.advance() {
            break;
        }
    }
}

#[inline]
fn _permute_no_alloc(
    whole_ordinals_stack: &mut [usize],
    split_ordinals_stack: &mut [usize],
    stack_start: usize,
    stack_len: usize,
    ordinals: &mut [usize],
    f: &mut impl FnMut(&[usize]) -> bool,
    depth: usize,
) -> bool {
    if stack_len != 0 {
        // SAFETY: by the nonoverlapping index ranges stack_start..stack_start+stack_len
        // and child_stack_start..child_stack_start+child_stack_len.
        let whole_ordinals_stack_shadow: *const _ = whole_ordinals_stack;
        let whole_ordinals_stack_shadow = unsafe { &*whole_ordinals_stack_shadow };
        let split_ordinals_stack_shadow: *mut _ = split_ordinals_stack;
        let split_ordinals_stack_shadow = unsafe { &mut *split_ordinals_stack_shadow };

        let mut splitter = SplitCombiner::new_no_alloc(CaptureMut::Borrowed(
            &mut split_ordinals_stack_shadow[stack_start..stack_start + stack_len - 1],
        ));

        // demarcation pointers for the next (child) frame of within whole_ordinals_stack
        let (child_stack_start, child_stack_len) = (stack_start + stack_len, stack_len - 1);
        loop {
            // the next child combination to recurse into
            let Split(head, tail) = splitter.read();

            // resolve the ordinals produced by the splitter into the actual ordinals in the context of the parent combination
            for (index, head_ordinal) in head
                .iter()
                .map(|head_ordinal| whole_ordinals_stack_shadow[stack_start + *head_ordinal])
                .enumerate()
            {
                whole_ordinals_stack[child_stack_start + index] = head_ordinal;
            }
            let tail_ordinal = whole_ordinals_stack[stack_start + tail];
            // println!(
            //     "{}{depth} — elements: {:?}, permuting split {:?}-{tail_ordinal}, ordinals: {:?}, {stack_start}|{stack_len}",
            //     "  ".repeat(depth),
            //     &whole_ordinals_stack[stack_start..stack_start + stack_len],
            //     &whole_ordinals_stack[child_stack_start..child_stack_start + child_stack_len],
            //     &ordinals[ordinals.len() - depth..]
            // );
            ordinals[ordinals.len() - depth - 1] = tail_ordinal;

            if !_permute_no_alloc(
                whole_ordinals_stack,
                split_ordinals_stack,
                child_stack_start,
                child_stack_len,
                ordinals,
                f,
                depth + 1,
            ) {
                return false;
            }

            if !splitter.advance() {
                break;
            }
        }
        true
    } else {
        //println!("{} completed ordinals: {ordinals:?}", "  ".repeat(depth));
        f(&ordinals)
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::sticky_permuter::permute;
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
        let expected_outputs = vec![[]];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permute_1p0() {
        let outputs = iterate_sticky(1, 0);
        let expected_outputs = vec![[]];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permute_1p1() {
        let outputs = iterate_sticky(1, 1);
        let expected_outputs = vec![[0]];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }

    #[test]
    fn permute_4p0() {
        let outputs = iterate_sticky(4, 0);
        let expected_outputs = vec![[]];
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
            [3, 2],
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
            [3, 2, 1],
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
            [3, 2, 1, 0],
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
        let expected_outputs = vec![[0, 1], [1, 0], [0, 2], [2, 0], [0, 3], [3, 0]];
        assert_eq!(inner_array_to_vec(expected_outputs), outputs);
    }
}
