/// Unopinionated extention of Option by another [`Some`] value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DoubleOption<T, U> {
    Fist(T),
    Second(U),
    Empty
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