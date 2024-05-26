use std::fmt::{Display, Formatter};

use thiserror::Error;
use vg_errortools::FatIOError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Misc error: {0}")]
    Misc(String),
    #[error("Error opening file: {0}")]
    IOError(#[from] FatIOError),
    #[error("Error parsing first json: {0}")]
    JSON(#[from] serde_json::Error),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Misc(value)
    }
}

#[derive(Debug)]
pub enum DiffType {
    RootMismatch,
    LeftExtra,
    RightExtra,
    Mismatch,
}

impl Display for DiffType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            DiffType::RootMismatch => "Mismatch at root.",
            DiffType::LeftExtra => "Extra on left",
            DiffType::RightExtra => "Extra on right",
            DiffType::Mismatch => "Mismatched",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathElement<'a> {
    Object(&'a str),
    ArrayEntry(usize),
}

impl<'a> PathElement<'a> {
    pub fn resolve<'b>(&self, v: &'b serde_json::Value) -> Option<&'b serde_json::Value> {
        match self {
            PathElement::Object(o) => v.get(o),
            PathElement::ArrayEntry(i) => v.get(*i),
        }
    }

    pub fn resolve_mut<'b>(
        &self,
        v: &'b mut serde_json::Value,
    ) -> Option<&'b mut serde_json::Value> {
        match self {
            PathElement::Object(o) => v.get_mut(o),
            PathElement::ArrayEntry(i) => v.get_mut(*i),
        }
    }
}

/// A view on a single end-node of the [`DiffKeyNode`] tree.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiffEntry<'a> {
    pub path: Vec<PathElement<'a>>,
    pub values: Option<(&'a serde_json::Value, &'a serde_json::Value)>,
}

impl Display for DiffEntry<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for element in &self.path {
            write!(f, ".{element}")?;
        }
        if let Some((l, r)) = &self.values {
            if l != r {
                write!(f, ".({l} != {r})")?;
            } else {
                write!(f, ".({l})")?;
            }
        }
        Ok(())
    }
}

impl Display for PathElement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathElement::Object(o) => {
                write!(f, "{o}")
            }
            PathElement::ArrayEntry(l) => {
                write!(f, "[{l}]")
            }
        }
    }
}
