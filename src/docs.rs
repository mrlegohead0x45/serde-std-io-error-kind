//! This crate provides a way for you to serialize and deserialize the `std::io::ErrorKind` enum.
//! To use it:
//! 
//! ```
//! use serde_std_io_error_kind::StdIoErrorKindDef;
//! use serde::{Serialize, Deserialize};
//! 
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "StdIoErrorKindDef")]
//!    error_kind: std::io::ErrorKind,
//! }
//! ```
//! 
//! ```
//! use serde_std_io_error_kind::StdIoErrorKindDef;
//! use serde::{Serialize, Deserialize};
//! 
//! #[derive(Serialize, Deserialize)]
//! enum MyIoErrorKind {
//!    FileNotFound,
//!    PermissionDenied,
//!    Other(#[serde(with = "StdIoErrorKindDef")] std::io::ErrorKind),
//! }
//! ```