//! Support for dis-cons transforms.

use crate::con;
use crate::logic::con::Conjunction;
use crate::logic::dis::Disjunction;
use crate::logic::New;

pub mod group_score;

pub type DisCons<T> = Disjunction<Conjunction<T>>;

#[inline]
pub fn expand<T: Clone>(lhs: &T, rhs: &Disjunction<T>) -> DisCons<T> {
    let dis_cons_elements = rhs.iter().map(|disjunct| con![lhs.clone(), disjunct.clone()]);
    DisCons::new(dis_cons_elements)
}

#[cfg(test)]
mod tests {
    use crate::dis_cons::expand;
    use crate::{con, dis};

    #[test]
    fn expand_empty() {
        let dis_cons = expand(&"a", &dis![]);
        assert_eq!(dis![], dis_cons);
    }

    #[test]
    fn expand_one() {
        let dis_cons = expand(&"a", &dis!["b"]);
        assert_eq!(dis![con!["a", "b"]], dis_cons);
    }

    #[test]
    fn expand_many() {
        let dis_cons = expand(&"a", &dis!["b", "c", "d"]);
        assert_eq!(dis![con!["a", "b"], con!["a", "c"], con!["a", "d"]], dis_cons);
    }
}