//! # Library for comparing JSON data structures
//! ## Summary
//! Main entry points are [`compare_strs`] to compare string slices and [`compare_serde_values`] to compare already parse [`serde_json::Value`]
//! ## Example:
//! ```rust
//! use json_diff_ng::compare_strs;
//! let data1 = r#"["a",{"c": ["d","f"] },"b"]"#;
//! let data2 = r#"["b",{"c": ["e","d"] },"a"]"#;
//! let diffs = compare_strs(data1, data2, true, &[]).unwrap();
//! assert!(!diffs.is_empty());
//! let diffs = diffs.unequal_values.get_diffs();
//!
//! assert_eq!(diffs.len(), 1);
//! assert_eq!(
//!   diffs.first().unwrap().to_string(),
//!   r#".[0].c.[1].("f" != "e")"#
//! );
//! ```
//! ## How to handle the results
//! Results are returned in a triple of [`DiffTreeNode`] called [`Mismatch`].
//! The triple consists of values only on the left side, values only on the right side and values on both sides that differ.
//! Since tree traversal is not usually what you want to do on client side, [`DiffTreeNode`] offers [`DiffTreeNode::get_diffs`] to retrieve
//! a flat list of [`DiffEntry`] which is more easily usable. The path in the json is collapsed into a vector of [`PathElement`] which can be used to follow the diff.
//! Similarly, all diffs after an operation can be collected using [`Mismatch::all_diffs`].
//!
//!

pub mod enums;
pub mod mismatch;
pub mod process;
pub mod sort;
pub use enums::DiffEntry;
pub use enums::DiffTreeNode;
pub use enums::DiffType;
pub use enums::Error;
pub use mismatch::Mismatch;
pub use process::compare_serde_values;
pub use process::compare_strs;
pub type Result<T> = std::result::Result<T, Error>;
