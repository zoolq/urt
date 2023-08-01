#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use core::pin::Pin;

use self::Double::{This, That};

/// Unopinionated version of `Result` with options This and That. 
/// This means every function that exists for `This` als exists for `That`
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Double<T, U> {
    This(T),
    That(U)
}

impl<T, U> Double<T, U> {
    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////
    #[inline]
    pub fn is_this(&self) -> bool {
        match *self {
            This(_) => true,
            That(_) => false
        }
    }

    #[inline]
    pub fn is_that(&self) -> bool {
        match *self {
            This(_) => false,
            That(_) => true
        }
    }

    #[inline]
    pub fn is_this_and<F>(self, f: F) -> bool 
    where
        F: FnOnce(T) -> bool
    {
        match self {
            This(this) => f(this),
            That(_) => false
        }
    }

    #[inline]
    pub fn is_that_and<F>(self, f: F) -> bool 
    where
        F: FnOnce(U) -> bool
    {
        match self {
            This(_) => false,
            That(that) => f(that)
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Adapter for each variant
    /////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn this(self) -> Option<T> {
        match self {
            This(this) => Some(this),
            That(_) => None
        }
    }

    #[inline]
    pub fn that(self) -> Option<U> {
        match self {
            This(_) => None,
            That(that) => Some(that)
        }
    }

    #[must_use = "directly use a result without casting from `Double`"]
    #[inline]
    pub fn this_as_result<E>(self) -> Result<T, U> {
        match self {
            This(this) => Ok(this),
            That(that) => Err(that)
        }
    }

    #[must_use = "directly use a result without casting from `Double`"]
    #[inline]
    pub fn that_as_result<E>(self) -> Result<U, T> {
        match self {
            This(this) => Err(this),
            That(that) => Ok(that)
        }
    }

    #[inline]
    pub fn this_or<E>(self, err: E) -> Result<T, E> {
        match self {
            This(this) => Ok(this),
            _ => Err(err)
        }
    }

    #[must_use = "directly use a result without casting from `Double`"]
    #[inline]
    pub fn that_or<E>(self, err: E) -> Result<U, E> {
        match self {
            That(that) => Ok(that),
            _ => Err(err)
        }
    }

    #[must_use = "directly use a result without casting from `Double`"]
    #[inline]
    pub fn this_or_else<F, E>(self, f: F) -> Result<T, E> 
    where
        F: FnOnce() -> E
    {
        match self {
            This(this) => Ok(this),
            _ => Err(f())
        }
    }

    #[inline]
    pub fn that_or_else<F, E>(self, f: F) -> Result<U, E> 
    where
        F: FnOnce() -> E
    {
        match self {
            That(that) => Ok(that),
            _ => Err(f())
        }
    }

    #[inline]
    pub fn flip(self) -> Double<U, T> {
        match self {
            This(this) => That(this),
            That(that) => This(that)
        }
    }


    #[inline]
    pub fn switch(self) -> Double<U, T> {
        match self {
            This(this) => That(this),
            That(that) => This(that)
        }
    }
    /////////////////////////////////////////////////////////////////////////
    // Adapter for working with references
    /////////////////////////////////////////////////////////////////////////
    
    #[inline]
    pub fn as_ref(&self) -> Double<&T, &U> {
        match *self {
            This(ref this) => This(this),
            That(ref that) => That(that)
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Double<&mut T, &mut U> {
        match *self {
            This(ref mut this) => This(this),
            That(ref mut that) => That(that)
        }
    }

    #[inline]
    pub fn as_pin_ref(self: Pin<&Self>) -> Double<Pin<&T>, Pin<&U>> {
        unsafe {
            match *Pin::get_ref(self) {
                This(ref this) => This(Pin::new_unchecked(this)),
                That(ref that) => That(Pin::new_unchecked(that))
            }
        }
    }

    #[inline]
    pub fn as_pin_mut(self: Pin<&mut Self>) -> Double<Pin<&mut T>, Pin<&mut U>> {
        unsafe {
            match *Pin::get_unchecked_mut(self) {
                This(ref mut this) => This(Pin::new_unchecked(this)),
                That(ref mut that) => That(Pin::new_unchecked(that))
            }
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Getting to contained values
    /////////////////////////////////////////////////////////////////////////
    
    #[inline]
    #[track_caller]
    pub fn expect_this(self, msg: &str) -> T {
        match self {
            This(this) => this,
            _ => panic!("{}", msg)
        }
    }

    #[inline]
    #[track_caller]
    pub fn expect_that(self, msg: &str) -> U {
        match self {
            That(that) => that,
            _ => panic!("{}", msg)
        }
    }

    #[inline]
    #[track_caller]
    pub fn unwrap_this(self) -> T {
        match self {
            This(this) => this,
            _ => panic!("called `Double::unwrap_this() on a `That` value")
        }
    }

    #[inline]
    #[track_caller]
    pub fn unwrap_that(self) -> U {
        match self {
            That(that) => that,
            _ => panic!("called `Double::unwrap_that() on a `This` value")
        }
    }

    #[inline]
    pub fn unwrap_this_or(self, default: T) -> T {
        match self {
            This(this) => this,
            _ => default
        }
    }

    #[inline]
    pub fn unwrap_that_or(self, default: U) -> U {
        match self {
            That(that) => that,
            _ => default
        }
    }


    #[inline]
    pub fn unwrap_this_or_else<F>(self, f: F) -> T 
    where
        F: FnOnce() -> T
    {
        match self {
            This(this) => this,
            _ => f()
        }
    }

    #[inline]
    pub fn unwrap_that_or_else<F>(self, f: F) -> U 
    where
        F: FnOnce() -> U
    {
        match self {
            That(that) => that,
            _ => f()
        }
    }

    #[inline]
    pub fn unwrap_this_with<F>(self, f: F) -> T 
    where
        F: FnOnce(U) -> T
    {
        match self {
            This(this) => this,
            That(that) => f(that)
        }
    }

    #[inline]
    pub fn unwrap_that_with<F>(self, f: F) -> U 
    where
        F: FnOnce(T) -> U
    {
        match self {
            That(that) => that,
            This(this) => f(this)
        }
    }

    #[inline]
    pub fn unwrap_this_or_default(self) -> T 
    where
        T: Default
    {
        match self {
            This(this) => this,
            _ => T::default()
        }
    }

    #[inline]
    pub fn unwrap_that_or_default(self) -> U
    where
        U: Default
    {
        match self {
            That(that) => that,
            _ => U::default()
        }
    }

    #[inline]
    pub fn unwrap_to<F, G, O>(self, f: F, g: G) -> O
    where 
        F: FnOnce(T) -> O,
        G: FnOnce(U) -> O
    {
        match self {
            This(this) => f(this),
            That(that) => g(that)
        }
    }

    #[inline]
    pub fn unwrap_into<O>(self) -> O
    where 
        T: Into<O>,
        U: Into<O>
    {
        match self {
            This(this) => this.into(),
            That(that) => that.into()
        }
    }

    #[inline]
    pub fn unwrap_union<F, O, V, R>(self, other: Self, f: F) -> O
    where 
        F: FnOnce(T, U) -> O,
    {
        match (self, other) {
            (This(a), That(b)) => f(a, b),
            (That(a), This(b)) => f(b, a),
            (This(_), This(_)) => panic!("called `Double::unwrap_union()` on `This` and `This` variants"),
            (That(_), That(_)) => panic!("called `Double::unwrap_union()` on `That` and `That` variants")
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Transforming contained values
    /////////////////////////////////////////////////////////////////////////
    
    #[inline]
    pub fn map_this<F, O>(self, f: F) -> Double<O, U> 
    where
        F: FnOnce(T) -> O
    {
        match self {
            This(this) => This(f(this)),
            That(that) => That(that)
        }
    }

    #[inline]
    pub fn map_that<F, O>(self, f: F) -> Double<T, O> 
    where
        F: FnOnce(U) -> O
    {
        match self {
            This(this) => This(this),
            That(that) => That(f(that))
        }
    }
    
    #[inline]
    pub fn map<F, G, O, R>(self, f: F, g: G) -> Double<O, R> 
    where
        F: FnOnce(T) -> O,
        G: FnOnce(U) -> R
    {
        match self {
            This(this) => This(f(this)),
            That(that) => That(g(that))
        }
    }

}

impl<T: Clone, U: Clone> Clone for Double<T, U> {
    fn clone(&self) -> Self {
        match self {
            This(this) => This(this.clone()),
            That(that) => That(that.clone())
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (This(destination), This(source)) => destination.clone_from(source),
            (That(destination), That(source)) => destination.clone_from(source),
            (destination, source) => *destination = source.clone()
        }        
    }
}