//! Urt (unambiguous result types) extends the standart libraries [`Option`] and [`Result`] types by adding 
//! multiple additional types which may be useful to return. 

#![cfg_attr(not(feature = "str"), no_std)]
#![warn(rustdoc::broken_intra_doc_links)]
#![doc(test(attr(deny(warnings))))]

/// Adds the `ErrorOption` enum which acts like Result and Option in one with 
/// variants `Value`, `Empty` and `Error` 
pub mod erroroption;
/// Adds the `DoubleOption` enum which extends [`Option`] by adding a second value
pub mod doubleoption;
/// Adds the `Double` enum for unopinionated [`Result`]s.
pub mod double;

