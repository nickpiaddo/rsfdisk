// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Library-level error module.

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// A specialized [`Result`](std::result::Result) type for `rsfdisk`.
///
/// This typedef is generally used at the program-level to avoid writing out [`RsFdiskError`]
/// directly, and is, otherwise, a direct mapping to [`Result`](std::result::Result).
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, RsFdiskError>;

/// Library-level runtime errors.
///
/// This enum includes all variants of error types susceptible to occur in the library. Other, more
/// granular error types, are automatically converted to an `RsFdiskError` when needed.
///
/// # Examples
/// ----
///
/// ```
/// fn main() -> rsfdisk::Result<()> {
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RsFdiskError {}
