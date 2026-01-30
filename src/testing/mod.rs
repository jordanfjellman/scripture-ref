//! Testing utilities for scripture-ref
//!
//! This module provides property-based test generators and fixtures
//! for testing scripture reference parsing.
//!
//! # Usage in Tests
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use crate::testing::generators::*;
//!     use proptest::prelude::*;
//!     
//!     proptest! {
//!         #[test]
//!         fn my_test(book in arb_book()) {
//!             // test with any book
//!         }
//!     }
//! }
//! ```

#[cfg(test)]
pub mod generators;

#[cfg(test)]
pub use generators::*;
