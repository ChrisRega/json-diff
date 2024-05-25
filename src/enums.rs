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
pub enum PathElement {
    Object(String),
    ArrayEntry(usize),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiffEntry {
    pub path: Vec<PathElement>,
    pub values: Option<(String, String)>,
}

impl Display for DiffEntry {
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

impl Display for PathElement {
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
