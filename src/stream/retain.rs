//! [`Retain`] converts from a borrowed type to an owned representation in a manner similar to
//! [`ToOwned`]. Unlike the latter, however, [`Retain`] does not mandate that the owned type
//! implement the [`Borrow`] trait.
//!
//! The removal of the [`Borrow`] constraint makes [`Retain`] useful for converting to owned
//! types whose borrowed representations are not necessarily references. Specifically, the borrowed
//! type may be an owned struct containing a reference. For example, the owned type `Point`
//! may have a borrowed type `BorrowedPoint<'a>` This relationship cannot currently be expressed
//! with the [`ToOwned`] and [`Borrow`] traits, which are limited to references. (It requires
//! GATs to express properly.)

pub trait Retain {
    type Retained: 'static;

    fn retain(&self) -> Self::Retained;
}

impl<W: ToOwned + ?Sized> Retain for W where <W as ToOwned>::Owned: 'static {
    type Retained = W::Owned;

    #[inline]
    fn retain(&self) -> Self::Retained {
        self.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use crate::stream::retain::Retain;

    #[test]
    fn retain_num() {
        let original = 42;
        let borrowed: &i32 = &original;
        let owned: i32 = borrowed.retain();
        assert_eq!(original, owned);
    }
    
    #[test]
    fn retain_vec() {
        let original = vec![0, 1];
        let borrowed: &[i32] = original.borrow();
        let owned: Vec<i32> = borrowed.retain();
        assert_eq!(original, owned);
    }
    
    #[test]
    fn retain_string() {
        let original = String::from("one");
        let borrowed: &str = original.borrow();
        let owned: String = borrowed.retain();
        assert_eq!(original, owned);
    }

    #[derive(Debug, PartialEq)]
    struct FooOwned(i32);

    impl FooOwned {
        fn borrow(&self) -> FooBorrowed {
            FooBorrowed(&self.0)
        }
    }

    struct FooBorrowed<'a>(&'a i32);

    impl Retain for FooBorrowed<'_> {
        type Retained = FooOwned;

        fn retain(&self) -> Self::Retained {
            FooOwned(*self.0)
        }
    }

    #[test]
    fn retain_inner_ref() {
        let original = FooOwned(42);
        let borrowed: FooBorrowed = original.borrow();
        let owned: FooOwned = borrowed.retain();
        assert_eq!(original, owned);
    }
}