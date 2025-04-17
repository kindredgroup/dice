use std::fmt::{Display, Formatter};
use std::ops::Deref;
use crate::logic::{format_elements, IntoInner, VecWrapper, VecWrapperMut};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conjunction<T>(Vec<T>);

impl<T> Deref for Conjunction<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl<T> Default for Conjunction<T> {
    #[inline]
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<T: Display> Display for Conjunction<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_elements(&self.0, |item, f| write!(f, "({item})"),  |_| Ok(()), f)
    }
}

impl<T> VecWrapper for Conjunction<T> {
    type Item = T;

    #[inline]
    fn vec(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> VecWrapperMut for Conjunction<T> {
    #[inline]
    fn vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T> IntoInner for Conjunction<T> {
    type Item = T;

    #[inline]
    fn into_inner(self) -> Vec<Self::Item> {
        self.0
    }
}

#[macro_export]
macro_rules! con {
    () => (
        <crate::logic::con::Conjunction<_> as crate::logic::New<_>>::new(Vec::new())
    );
    ($($x:expr),+ $(,)?) => (
        <crate::logic::con::Conjunction<_> as crate::logic::New<_>>::new((vec![$($x),+]))
    );
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::logic::{New, Push};
    use crate::logic::con::Conjunction;

    #[test]
    fn conjunction_display() {
        let dis = con!["a", "b", "c"];
        assert_eq!("(a)(b)(c)", dis.to_string());
    }
    
    #[test]
    fn new_and_push() {
        let mut con = Conjunction::new(["a", "b", "c"]);
        assert_eq!(&["a", "b", "c"], con.deref());
        con.push("d");
        assert_eq!(&["a", "b", "c", "d"], con.deref());
    }
}