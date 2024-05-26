//! # Library for comparing JSON data structures
//! ## Summary
//! Main entry points are [`compare_strs`] to compare string slices and [`compare_serde_values`] to compare already parse [`serde_json::Value`]
//! ## Examples:
//! ### Compare string slices:
//! ```rust
//! use json_diff::compare_strs;
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
//!
//!

pub mod ds;

pub mod enums;
pub mod process;
pub use ds::key_node::TreeNode;
pub use enums::DiffEntry;
pub use enums::DiffType;
pub use enums::Error;
pub use process::compare_serde_values;
pub use process::compare_strs;
pub type Result<T> = std::result::Result<T, Error>;
