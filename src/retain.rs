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

// impl<W: Clone + 'static> Own for [W] {
//     type Owned = Vec<W>;
// 
//     #[inline]
//     fn own(&self) -> Self::Owned {
//         self.to_vec()
//     }
// }

// impl Own for str {
//     type Owned = String;
// 
//     #[inline]
//     fn own(&self) -> Self::Owned {
//         self.to_string()
//     }
// }

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use crate::retain::Retain;

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