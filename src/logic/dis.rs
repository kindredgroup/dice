use std::fmt::{Display, Formatter};
use std::ops::Deref;
use crate::logic::{format_elements, IntoInner, VecWrapper, VecWrapperMut};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Disjunction<T>(Vec<T>);

impl<T> Deref for Disjunction<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl<T> Default for Disjunction<T> {
    #[inline]
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<T: Display> Display for Disjunction<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_elements(&self.0, |item, f| write!(f, "{item}"),  |f| write!(f, " ∨ "), f)
    }
}

impl<T> VecWrapper for Disjunction<T> {
    type Item = T;

    #[inline]
    fn vec(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> VecWrapperMut for Disjunction<T> {
    #[inline]
    fn vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T> IntoInner for Disjunction<T> {
    type Item = T;

    #[inline]
    fn into_inner(self) -> Vec<Self::Item> {
        self.0
    }
}

#[macro_export]
macro_rules! dis {
    () => (
        <crate::logic::dis::Disjunction<_> as crate::logic::New<_>>::new(Vec::new())
    );
    ($($x:expr),+ $(,)?) => (
        <crate::logic::dis::Disjunction<_> as crate::logic::New<_>>::new((vec![$($x),+]))
    );
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::logic::dis::Disjunction;
    use crate::logic::{New, Push};

    #[test]
    fn disjunction_display() {
        let dis = dis!["a", "b", "c"];
        assert_eq!("a ∨ b ∨ c", dis.to_string());
    }
    
    #[test]
    fn new_and_push() {
        let mut dis = Disjunction::new(["a", "b", "c"]);
        assert_eq!(&["a", "b", "c"], dis.deref());
        dis.push("d");
        assert_eq!(&["a", "b", "c", "d"], dis.deref());
    }
}