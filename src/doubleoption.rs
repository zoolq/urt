use self::DoubleOption::{First, Second, Empty};

/// Unopinionated extention of Option by another [`Some`] value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DoubleOption<T, U> {
    First(T),
    Second(U),
    Empty
}

impl<T, U> DoubleOption<T, U>{
    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////
    #[inline]
    pub fn is_fist(&self) -> bool {
        match *self {
            First(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_second(&self) -> bool {
        match *self {
            Second(_) => true,
            _ => false
        }
    }

    #[inline]
    fn is_first_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(T) -> bool
    {
        match *self {
            First(first) => f(first),
            _ => false
        }
    }

    #[inline]
    fn is_second_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(U) -> bool
    {
        match *self {
            Second(sec) => f(sec),
            _ => false
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        match *self {
            Empty => true,
            _ => false
        }
    }


}


impl<T: Clone, U: Clone> Clone for DoubleOption<T, U> {
    fn clone(&self) -> Self {
        match self {
            Self::Fist(first) => Self::Fist(first.clone()),
            Self::Second(second) => Self::Second(second.clone()),
            Self::Empty => Self::Empty
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Self::Fist(destination), Self::Fist(source)) => destination.clone_from(source),
            (Self::Second(destination), Self::Second(source)) => destination.clone_from(source),
            (destination, source) => *destination = source.clone()
        }
    }
}