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
    pub fn is_first_and<F>(self, f: F) -> bool
    where
        F: FnOnce(T) -> bool
    {
        match self {
            First(first) => f(first),
            _ => false
        }
    }

    #[inline]
    pub fn is_second_and<F>(self, f: F) -> bool
    where
        F: FnOnce(U) -> bool
    {
        match self {
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

    /////////////////////////////////////////////////////////////////////////
    // Adapter for working with references
    /////////////////////////////////////////////////////////////////////////
    #[inline]
    pub fn as_ref(&self) -> DoubleOption<&T, &U> {
        match self {
            First(ref first) => First(&first),
            Second(ref sec) => Second(&sec),
            Empty => Empty
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> DoubleOption<&mut T, &mut U> {
        match self {
            First(ref mut first) => First(first),
            Second(ref mut sec) => Second(sec),
            Empty => Empty
        }
    }
}


impl<T: Clone, U: Clone> Clone for DoubleOption<T, U> {
    fn clone(&self) -> Self {
        match self {
            First(first) => First(first.clone()),
            Second(second) => Second(second.clone()),
            Empty => Empty
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (First(destination), First(source)) => destination.clone_from(source),
            (Second(destination), Second(source)) => destination.clone_from(source),
            (destination, source) => *destination = source.clone()
        }
    }
}