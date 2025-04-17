//! Support for dis-cons transforms.

use crate::con;
use crate::logic::con::Conjunction;
use crate::logic::dis::Disjunction;
use crate::logic::{New, Push};

pub mod group_score;

pub type DisCons<T> = Disjunction<Conjunction<T>>;

#[inline]
pub fn expand_dis<T: Clone>(lhs: &T, rhs: &Disjunction<T>) -> DisCons<T> {
    expand(lhs, rhs, |lhs, disjunct| con![lhs.clone(), disjunct.clone()])
}

#[inline]
pub fn expand_dis_cons<T: Clone>(lhs: &T, rhs: &DisCons<T>) -> DisCons<T> {
    expand(lhs, rhs, |lhs, disjunct| {
        let mut conjunction = Conjunction::with_capacity(disjunct.len() + 1);
        conjunction.push(lhs.clone());
        for conjunct in disjunct.iter() {
            conjunction.push(conjunct.clone());
        }
        conjunction
    })
}

#[inline]
fn expand<T, U, V, F>(lhs: &T, rhs: &Disjunction<U>, f: F) -> Disjunction<V> where F: Fn(&T, &U) -> V {
    Disjunction::new(rhs.iter().map(|disjunct| f(lhs, disjunct)))
}

#[cfg(test)]
mod tests {
    use crate::dis_cons::{expand_dis, expand_dis_cons};
    use crate::{con, dis};

    #[test]
    fn expand_dis_empty() {
        let dis_cons = expand_dis(&"a", &dis![]);
        assert_eq!(dis![], dis_cons);
    }

    #[test]
    fn expand_dis_one() {
        let dis_cons = expand_dis(&"a", &dis!["b"]);
        assert_eq!(dis![con!["a", "b"]], dis_cons);
    }

    #[test]
    fn expand_dis_many() {
        let dis_cons = expand_dis(&"a", &dis!["b", "c", "d"]);
        assert_eq!(dis![con!["a", "b"], con!["a", "c"], con!["a", "d"]], dis_cons);
    }

    #[test]
    fn expand_dis_cons_empty() {
        let dis_cons = expand_dis_cons(&"a", &dis![]);
        assert_eq!(dis![], dis_cons);
    }

    #[test]
    fn expand_dis_cons_one() {
        let dis_cons = expand_dis_cons(&"a", &dis![con!["b", "c"]]);
        assert_eq!(dis![con!["a", "b", "c"]], dis_cons);
    }

    #[test]
    fn expand_dis_cons_many() {
        let dis_cons = expand_dis_cons(&"a", &dis![con!["b", "c"], con!["d", "e"]]);
        assert_eq!(dis![con!["a", "b", "c"], con!["a", "d", "e"]], dis_cons);
    }
}