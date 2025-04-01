use crate::comb::bitmap::Bitmap;

pub fn permute(n: usize, r: usize) {
    let elements = Bitmap::full(n);
    let stack = vec![];
    _permute(&elements, &stack, 0);
}

fn _permute(elements: &Bitmap, stack: &[usize], depth: usize) {
    if elements.size() > 1 {
        for Split(head, tail) in Splitter::new(&elements) {
            println!("{}permuting split {head}-{tail}, stack: {stack:?}", "  ".repeat(depth));
            let mut new_stack = Vec::with_capacity(stack.len() + 1);
            new_stack.push(tail);
            for ordinal in stack {
                new_stack.push(*ordinal);
            }
            _permute(&head, &new_stack, depth + 1)
        }
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
    
    #[test]
    fn permute_4() {
        permute(4, 4);
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

// #[inline]
// fn split(bitmap: &Bitmap) -{
//     
// }