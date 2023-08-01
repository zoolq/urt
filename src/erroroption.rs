#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(feature = "serde")]
extern crate serde;

use core::{
    iter::FusedIterator,
    ops::{Deref, DerefMut},
    default::Default,
    marker::Copy,
    hint, mem, fmt::Debug  
};

use self::ErrorOption::{Value, Empty, Error};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[must_use = "this `ErrorOption` may be an `Error` variant, which should be handeled"]
#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
/// Opinionated extention of Option with an additional Error variant
pub enum ErrorOption<T, E> {
    /// Some success value of type `T`.
    Value(T),
    /// No value.
    Empty,
    /// Error value of type `E`.
    Error(E)
}

impl<T, E> ErrorOption<T, E> {
    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the `ErrorOption` is `Value`.
    /// 
    /// # Examples 
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Error, Empty};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.is_value(), true);
    /// 
    /// let bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.is_value(), false);
    /// 
    /// let baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.is_value(), false);
    /// ```
    #[inline]
    pub const fn is_value(&self) -> bool {
        matches!(self, Value(_))
    }
    
    /// Returns `true` if the `ErrorOption` is `Value` and the predicate is meet.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Error};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.is_value_and(|x| x > 10), true);
    /// 
    /// let bar: ErrorOption<i32, &str> = Value(2);
    /// assert_eq!(bar.is_value_and(|x| x > 10), false);
    /// 
    /// let baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.is_value_and(|x| x > 10), false);
    /// ```
    #[inline]
    pub fn is_value_and<F>(self, f: F) -> bool
    where
        F: FnOnce(T) -> bool
    {
        match self {
            Value(value) => f(value),
            _ => false
        }
    }

    /// Returns `true` if the `ErrorOption` is `Empty`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Error, Empty};
    /// let foo: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(foo.is_empty(), true);
    /// 
    /// let bar: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(bar.is_empty(), false);
    /// 
    /// let baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.is_empty(), false);
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }

    /// Returns `true` if the `ErrorOption` is `Error`.
    ///     
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Error, Empty};
    /// let foo: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(foo.is_error(), true);
    /// 
    /// let bar: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(bar.is_error(), false);
    /// 
    /// let baz: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(baz.is_error(), false);
    /// ```
    #[inline]
    pub const fn is_error(&self) -> bool {
        matches!(self, Error(_))
    }

    /// Returns `true` if the `ErrorOption` is `Error` and the predicate is meet.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Error};
    /// let foo: ErrorOption<i32, &str> = Error("error");
    /// assert_eq!(foo.is_error_and(|e| e == "error"), true);
    /// 
    /// let bar: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(bar.is_error_and(|e| e == "error"), false);
    /// 
    /// let baz: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(baz.is_error_and(|e| e == "error"), false);
    /// ```
    #[inline]
    pub fn is_error_and<F>(self, f: F) -> bool 
    where
        F: FnOnce(E) -> bool
    {
        match self {
            Error(error) => f(error),
            _ => false
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Adapter for each variant
    /////////////////////////////////////////////////////////////////////////

    /// Maps `ErrorOption` to [`Option`], where `Error` and `Empty` map to [`None`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Error, Empty};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.as_option(), Some(42));
    /// 
    /// let bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.as_option(), None);
    /// 
    /// let baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.as_option(), None);
    /// ```
    #[inline]
    pub fn as_option(self) -> Option<T> {
        match self {
            Value(value) => Some(value),
            _ => None
        }
    }

    /// Maps `ErrorOption` to [`Result`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Error, Empty};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.as_result(), Ok(42));
    /// 
    /// let bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.as_result(), Ok(0));
    /// 
    /// let baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.as_result(), Err("This is an error!"));
    /// ```
    #[inline]
    pub fn as_result(self) -> Result<T, E> 
    where
        T: Default
    {
        match self {
            Value(value) => Ok(value),
            Empty => Ok(T::default()),
            Error(error) => Err(error)
        }
    }

    /// Maps `ErrorOption` to [`Option`], where `Value` and `Empty` map to [`None`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Error, Empty};
    /// let foo: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(foo.error(), Some("This is an error!"));
    /// 
    /// let bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.error(), None);
    /// 
    /// let baz: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(baz.error(), None);
    /// ```
    #[inline]
    pub fn error(self) -> Option<E> {
        match self {
            Error(error) => Some(error),
            _ => None
        }
    }

    /// Maps `ErrorOption` to [`Result`] using `err` as [`Err`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Error, Empty};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.value_or("Default error"), Ok(42));
    /// 
    /// let bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.value_or("Default error"), Err("Default error"));
    /// 
    /// let baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.value_or("Default error"), Err("Default error"));
    /// ```
    #[inline]
    pub fn value_or<O>(self, err: O) -> Result<T, O> {
        match self {
            Value(value) => Ok(value),
            _ => Err(err)
        }
    }

    /// Maps `ErrorOption` to [`Result`] defaulting to `err`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Error, Empty};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.value_or_default("Default error"), Ok(42));
    /// 
    /// let bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.value_or_default("Default error"), Err("Default error"));
    /// 
    /// let baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.value_or_default("Default error"), Err("This is an error!"));
    /// ```
    #[inline]
    pub fn value_or_default(self, err: E) -> Result<T, E> {
        match self {
            Value(value) => Ok(value),
            Empty => Err(err),
            Error(error) => Err(error)
        }
    }

    /// Maps `ErrorOption` to [`Result`] casting `Value` to [`Ok`], defaulting to `f`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Empty};
    /// let error = || "This is an error!";
    /// 
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.value_or_else(error), Ok(42));
    /// 
    /// let bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.value_or_else(error), Err("This is an error!"))
    /// ```
    #[inline]
    pub fn value_or_else<F, O>(self, f: F) -> Result<T, O> 
    where
        F: FnOnce() -> O
    {
        match self {
            Value(value) => Ok(value),
            _ => Err(f())
        }
    }

    /// Maps `ErrorOption` to [`Result`] turning `Value` and `Empty` into an [`Option`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Empty, Error};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.result(), Ok(Some(42)));
    /// 
    /// let bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.result(), Ok(None));
    /// 
    /// let baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.result(), Err("This is an error!"));
    /// ```
    #[inline]
    pub fn result(self) -> Result<Option<T>, E> {
        match self {
            Value(value) => Ok(Some(value)),
            Empty => Ok(None),
            Error(error) => Err(error)
        }
    }

    /// Switches `Value` and `Error`, keeping `Empty`.
    /// 
    /// This method in not recomended due to the inherent biased nature of `ErrorOption` if you want to use a more
    /// unbiased type use [`Double`] or [`DoubleOption`] instead.
    /// 
    /// [`Double`]: urt::double::Double
    /// [`DoubleOption`]: urt::doubleoption::DoubleOption
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Empty, Error};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.switch(), Error(42));
    /// 
    /// let bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.switch(), Empty);
    /// 
    /// let baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.switch(), Value("This is an error!"));
    /// ```
    #[inline]
    pub fn switch(self) -> ErrorOption<E, T> {
        match self {
            Value(value) => Error(value),
            Empty => Empty,
            Error(error) => Value(error),
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Adapter for working with references
    /////////////////////////////////////////////////////////////////////////
    
    /// Converts from `&ErrorOption<T, E>` to `ErrorOption<&T, &E>`.
    ///
    /// Produces a new `ErrorOption`, containing a reference
    /// into the original, leaving the original in place.
    ///
    /// # Examples
    ///
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Empty, Error};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.as_ref(), Value(&42));
    /// 
    /// let bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.as_ref(), Empty);
    /// 
    /// let baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.as_ref(), Error(&"This is an error!"));
    /// ```
    #[inline]
    pub const fn as_ref(&self) -> ErrorOption<&T, &E> {
        match self {
            Value(ref value) => Value(value),
            Empty => Empty,
            Error(ref error) => Error(error)
        }
    }

    /// Converts from `&mut ErrorOption<T, E>` to `ErrorOption<&mut T, &mut E>`.
    ///
    /// Produces a new `ErrorOption`, containing a reference
    /// into the original, leaving the original in place.
    ///
    /// # Examples
    ///
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value, Empty, Error};
    /// let mut foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.as_mut(), Value(&mut 42));
    /// 
    /// let mut bar: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(bar.as_mut(), Empty);
    /// 
    /// let mut baz: ErrorOption<i32, &str> = Error("This is an error!");
    /// assert_eq!(baz.as_mut(), Error(&mut "This is an error!"));
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> ErrorOption<&mut T, &mut E> {
        match self {
            Value(ref mut value) => Value(value),
            Empty => Empty,
            Error(ref mut error) => Error(error)
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Getting to contained values
    /////////////////////////////////////////////////////////////////////////

    /// Returns the contained `Value`, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the `Error` and `Empty`
    /// variants explicitly, or call [`unwrap_or`], [`unwrap_or_else`], or
    /// [`unwrap_or_default`].
    ///
    /// [`unwrap_or`]: ErrorOption::unwrap_or
    /// [`unwrap_or_else`]: ErrorOption::unwrap_or_else
    /// [`unwrap_or_default`]: ErrorOption::unwrap_or_default
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Error` or `Empty`, with the passed message 
    /// and the error if present.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use urt::erroroption::ErrorOption::{self, Error};
    /// let foo: ErrorOption<i32, &str> = Error("emergency failure");
    /// foo.expect("Testing expect"); // panics with `Testing expect: emergency failure`
    /// ```
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T 
    where
        E: Debug
    {
        match self {
            Value(value) => value,
            #[cfg(not(feature = "panic_immediate_abort"))]
            Error(error) => panic!("{msg}: {error:?}"),
            _ => panic!()
        }
    }

    /// Returns the contained `Error`, consuming the `self` value.
    /// 
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the `Error` and `Empty`
    /// variants explicitly.
    ///
    /// # Panics
    ///
    /// Panics if the passed value is `Value` or `Empty`, with the passed message 
    /// and value if present.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use urt::erroroption::ErrorOption::{self, Value};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// foo.expect_error("Testing expect"); // panics with `Testing expect: 42`
    /// ```
    #[inline]
    #[track_caller]
    pub fn expect_error(self, msg: &str) -> E 
    where
        T: Debug
    {
        match self {
            Error(error) =>  error,
            #[cfg(not(feature = "panic_immediate_abort"))]
            Value(error) => panic!("{msg}: {error:?}"),
            _ => panic!()
        }
    }

    /// Returns the contained `Value`, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the `Error` and `Empty`
    /// variants explicitly, or call [`unwrap_or`], [`unwrap_or_else`], or
    /// [`unwrap_or_default`].
    ///
    /// [`unwrap_or`]: ErrorOption::unwrap_or
    /// [`unwrap_or_else`]: ErrorOption::unwrap_or_else
    /// [`unwrap_or_default`]: ErrorOption::unwrap_or_default
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Error` or `Empty`.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use urt::erroroption::ErrorOption::{self, Error};
    /// let foo: ErrorOption<i32, &str> = Error("emergency failure");
    /// foo.unwrap(); // panics
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Value(value) => value,
            Empty => panic!("called `ErrorOption::unwrap()` on a `Empty` value"),
            Error(_) => panic!("called `ErrorOption::unwrap()` on a `Error` value")
        }
    }

    /// Returns the contained `Error`, consuming the `self` value.
    /// 
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the `Error` and `Empty`
    /// variants explicitly.
    ///
    /// # Panics
    ///
    /// Panics if the passed value is `Value` or `Empty`, with the passed message.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use urt::erroroption::ErrorOption::{self, Value};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// foo.unwrap_error(); // panics
    /// ```
    #[inline]
    #[track_caller]
    pub fn unwrap_error(self) -> E {
        match self {
            Error(error) => error,
            Empty => panic!("called ErrorOption::unwrap_error() on a `Empty` value"),
            Value(_) => panic!("called ErrorOption::unwrap_error() on a `Value` value")
        }
    }

    /// Returns the contained `Value`, or if `self` is `Error` or `Empty` returns the passed default, consuming the `self` value.
    /// 
    /// # Examples
    ///
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Error, Value, Empty};
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.unwrap_or(0), 42);
    /// 
    /// let bar: ErrorOption<i32, &str> = Error("This gets discarded!");
    /// assert_eq!(bar.unwrap_or(0), 0);
    /// 
    /// let baz: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(baz.unwrap_or(0), 0);
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Value(value) => value,
            _ => default
        }
    }

    /// Returns the contained `Value`, or if `self` is `Error` or `Empty` returns the passed closure, consuming the `self` value.
    /// 
    /// # Examples
    ///
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Error, Value, Empty};
    /// let default = || 12 + 30;
    /// 
    /// let foo: ErrorOption<i32, &str> = Value(42);
    /// assert_eq!(foo.unwrap_or_else(default), 42);
    /// 
    /// let bar: ErrorOption<i32, &str> = Error("This gets discarded!");
    /// assert_eq!(bar.unwrap_or_else(default), 42);
    /// 
    /// let baz: ErrorOption<i32, &str> = Empty;
    /// assert_eq!(baz.unwrap_or_else(default), 42);
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T 
    where
        F: FnOnce() -> T
    {
        match self {
            Value(value) => value,
            _ => f()
        }
    }

    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default
    {
        match self {
            Value(value) => value,
            _ => T::default()
        }
    }

    #[inline]
    #[track_caller]
    pub unsafe fn unwrap_unchecked(self) -> T {
        debug_assert!(self.is_value());
        match self {
            Value(value) => value,
            _ => unsafe { hint::unreachable_unchecked() }
        }
    }

    /////////////////////////////////////////////////////////////////////////
    // Transforming contained values
    /////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn map<M, F>(self, f: F) -> ErrorOption<M, E> 
    where
        F: FnOnce(T) -> M
    {
        match self {
            Value(value) => Value(f(value)),
            Empty => Empty,
            Error(error) => Error(error)
        }
    }

    #[inline]
    pub fn map_or<M, F>(self, default: M, f: F) -> M 
    where
        F: FnOnce(T) -> M
    {
        match self {
            Value(value) => f(value),
            _ => default
        }
    }

    #[inline]
    pub fn map_or_else<M, D, F>(self, default: D, f: F) -> M
    where
        D: FnOnce() -> M,
        F: FnOnce(T) -> M
    {
        match self {
            Value(value) => f(value),
            _ => default()
        }
    }

    #[inline]
    pub fn map_or_error<M, DE, D, F>(self, default_error: DE, default: D, f: F) -> M
    where
        DE: FnOnce(E) -> M,
        D: FnOnce() -> M,
        F: FnOnce(T) -> M
    {
        match self {
            Value(value) => f(value),
            Empty => default(),
            Error(error) => default_error(error)
        }
    }

    #[inline]
    pub fn map_error<F, O>(self, f: F) -> ErrorOption<T, O>
    where
        F: FnOnce(E) -> O
    {
        match self {
            Value(value) => Value(value),
            Empty => Empty,
            Error(error) => Error(f(error))
        }
    }

    #[inline]
    pub fn inspect<F>(self, f: F) -> Self 
    where
        F: FnOnce(&T)
    {
        if let Value(ref value) = self {
            f(value)
        }

        self
    }

    #[inline]
    pub fn inspect_err<F>(self, f: F) -> Self 
    where
        F: FnOnce(&E) 
    {
        if let Error(ref error) = self {
            f(error)
        }

        self
    }

    #[inline]
    pub fn as_deref(&self) -> ErrorOption<&T::Target, &E>
    where
        T: Deref
    {
        match self {
            Value(ref value) => Value(value.deref()),
            Empty => Empty,
            Error(ref error) => Error(error)
        }
    }

    #[inline]
    pub fn as_deref_mut(&mut self) -> ErrorOption<&mut T::Target, &mut E>
    where
        T: DerefMut
    {
        match self {
            Value(ref mut value) => Value(value.deref_mut()),
            Empty => Empty,
            Error(ref mut error) => Error(error)
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { inner: self.as_ref().as_option() }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { inner: self.as_mut().as_option() }
    }


    #[inline]
    pub fn and<M>(self, optb: ErrorOption<M, E>) -> ErrorOption<M, E> {
        match self {
            #[allow(unused_variables)]
            Value(value) => optb,
            Empty => Empty,
            Error(error) => Error(error)
        }
    }

    #[inline]
    pub fn and_then<M, F>(self, f: F) -> ErrorOption<M, E> 
    where
        F: FnOnce(T) -> ErrorOption<M, E>
    {
        match self {
            Value(value) => f(value),
            Empty => Empty,
            Error(error) => Error(error)
        }
    }

    #[inline]
    pub fn filter<P>(self, predicate: P) -> Self 
    where
        P: FnOnce(&T) -> bool
    {
        if let Value(value) = self {
            if predicate(&value) {
                return Value(value);
            }
        }
        Empty
    }

    #[inline]
    pub fn filter_or<P>(self, predicate: P, default: Self) -> Self 
    where
        P: FnOnce(&T) -> bool
    {
        if let Value(value) = self {
            if predicate(&value) {
                return Value(value);
            }
        }
        default
    }

    #[inline]
    pub fn filter_or_else<P, F>(self, predicate: P, default: F) -> Self 
    where
        P: FnOnce(&T) -> bool,
        F: FnOnce() -> Self
    {
        if let Value(value) = self {
            if predicate(&value) {
                return Value(value);
            }
        }
        default()
    }

    pub fn filter_predicate<P>(self, predicate: P) -> Self 
    where
        P: FnOnce(&Self) -> bool
    {
        if predicate(&self) {
            return self;
        }
        Empty
    }

    #[inline]
    pub fn filter_predicate_or<P>(self, predicate: P, default: Self) -> Self 
    where
        P: FnOnce(&Self) -> bool,
    {
        if predicate(&self) {
            return self;
        }
        default
    }

    #[inline]
    pub fn filter_predicate_or_else<P, D>(self, predicate: P, default: D) -> Self 
    where
        P: FnOnce(&Self) -> bool,
        D: FnOnce(&Self) -> Self
    {
        if predicate(&self) {
            return self;
        }
        default(&self)
    }

    #[inline]
    pub fn or(self, optb: Self) -> Self {
        match self {
            Value(value) => Value(value),
            _ => optb
        }
    }

    #[inline]
    pub fn or_else<F>(self, f: F) -> Self 
    where
        F: FnOnce() -> Self
    {
        match self {
            Value(value) => Value(value),
            _ => f()
        }
    }

    #[inline]
    pub fn xor(self, optb: Self) -> Self {
        match (self, optb) {
            (Value(value), Error(_)) | (Value(value), Empty) => Value(value),
            (Error(_), Value(value)) | (Empty, Value(value)) => Value(value),
            _ => Empty
        }
    }

    #[must_use = "if you intended to set a value, consider assignment instead"]
    #[inline]
    pub fn insert(&mut self, value: T) -> &mut T {
        *self = Value(value);
        unsafe { self.as_mut().unwrap_unchecked() }
    } 

    #[inline]
    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        if !self.is_value() {
            *self = Value(value);
        }
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub fn get_or_insert_default(&mut self) -> &mut T 
    where 
        T: Default
    {
        self.get_or_insert(T::default())
    }

    #[inline]
    pub fn get_or_insert_with<F>(&mut self, f: F) -> &mut T 
    where
        F: FnOnce() -> T
    {
        if !self.is_value() {
            mem::forget(mem::replace(self, Value(f())));
        }

        unsafe { self.as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub fn take(&mut self) -> Self {
        mem::take(self)
    }

    #[inline]
    pub fn replace(&mut self, value: T) -> Self {
        mem::replace(self, Value(value))
    }

    // ToDo: add zip_to_error and with error (need to figure out what to do with conflicting errors)
    // Does it make sense to prioratize the first error and only porpagate the 2nd if the first has a value?
    // Should we only ever propagate the first? Should this panic and have a function where a defaut error is provided?

    #[inline]
    pub fn zip<U>(self, other: ErrorOption<U, E>) -> ErrorOption<(T, U), E> {
        match (self, other) {
            (Value(a), Value(b)) => Value((a, b)),
            // Should this drop the error or propagate it? Should this return any error if one is present if so which?
            _ => Empty
        }
    }

    #[inline]
    pub fn zip_to_option<U>(self, other: ErrorOption<U, E>) -> Option<(T, U)> {
        match (self, other) {
            (Value(a), Value(b)) => Some((a, b)),
            _ => None
        }
    }

    #[inline]
    pub fn zip_with_option<U>(self, other: Option<U>) -> Option<(T, U)> {
        match (self, other) {
            (Value(a), Some(b)) => Some((a, b)),
            _ => None
        }
    }

    #[inline]
    pub fn zip_with<U, F, R>(self, other: ErrorOption<U, T>, f: F) -> ErrorOption<R, E>
    where
        F: FnOnce(T, U) -> R
    {
        match (self, other) {
            (Value(a), Value(b)) => Value(f(a, b)),
            // look at zip
            _ => Empty
        }
    }

    #[inline]
    pub fn zip_to_option_with<U, F, R>(self, other: ErrorOption<U, T>, f: F) -> Option<R>
    where
        F: FnOnce(T, U) -> R
    {
        match (self, other) {
            (Value(a), Value(b)) => Some(f(a, b)),
            _ => None
        }
    }


    // ToDo: 
    //
    // Iterators and Special Optimized traits
    //
    // General:
    // Do something about nested ErrorOptions
    // Fix unwrap_failed to be 2 methods
    // Implement iterators
}

impl<T, U ,E> ErrorOption<(T, U), E> {
    /// Converts an `ErrorOption<(T, U), E>` to `(ErrorOption<T, E>, ErrorOption<U, E>)`
    /// 
    /// #Examples
    /// 
    /// ```
    /// # use urt::erroroption::ErrorOption::{self, Value};
    /// let ziped: ErrorOption<(i32, u32), &str> = Value((-12, 2));
    /// assert_eq!(ziped.unzip(), (Value(-12), Value(2)));
    /// ```
    #[inline]
    pub fn unzip(self) -> (ErrorOption<T, E>, ErrorOption<U, E>) {
        match self {
            Value((a, b)) => (Value(a), Value(b)),
            _ => (Empty, Empty),
        }
    }
}

impl<T, E> ErrorOption<&T, E> {
    #[inline]
    pub fn copied(self) -> ErrorOption<T, E>
    where
        T: Copy
    {
        self.map(|&t| t)
    }

    #[inline]
    pub fn cloned(self) -> ErrorOption<T, E>
    where
        T: Clone
    {
        self.map(|t| t.clone())
    }
}

impl<T, E> ErrorOption<&mut T, E> {
    #[inline]
    pub fn copied(self) -> ErrorOption<T, E>
    where
        T: Copy
    {
        self.map(|&mut t| t)
    }

    #[inline]
    pub fn cloned(self) -> ErrorOption<T, E>
    where
        T: Clone
    {
        self.map(|t| t.clone())
    }
}

impl<T, E> Default for ErrorOption<T, E> {
    #[inline]
    fn default() -> ErrorOption<T, E> {
        Empty
    }
}

impl<T: Clone, E: Clone> Clone for ErrorOption<T, E> {
    fn clone(&self) -> Self {
        match self {
            Value(value) => Value(value.clone()),
            Error(error) => Error(error.clone()),
            Empty => Empty,
        }
    }

    fn clone_from(&mut self, source: &Self) { 
        match (self, source) {
            (Value(destination), Value(source)) => destination.clone_from(source),
            (Error(destination), Error(source)) => destination.clone_from(source),
            (destination, source) => *destination = source.clone(),
        }
    }
}



impl<T, E> IntoIterator for ErrorOption<T, E> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { inner: self.as_option() }
    }
}

impl<'a, T, E> IntoIterator for &'a ErrorOption<T, E> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, E> IntoIterator for &'a mut ErrorOption<T, E> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/////////////////////////////////////////////////////////////////////////////
// The ErrorOption Iterators
/////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
    inner: Option<&'a T>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.take()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.take()
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for Iter<'_, T> {}

impl<T> Clone for Iter<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Iter { inner: self.inner }
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
    inner: Option<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        self.inner.take()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        self.inner.take()
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {}

impl<T> FusedIterator for IterMut<'_, T> {}

#[derive(Clone, Debug)]
pub struct IntoIter<T> {
    inner: Option<T>
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.take()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.take()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}