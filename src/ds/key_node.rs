use std::collections::HashMap;

use serde_json::Value;

use crate::enums::{DiffEntry, PathElement};

#[derive(Debug, PartialEq)]
pub enum DiffTreeNode {
    Null,
    Value(Value, Value),
    Node(HashMap<String, DiffTreeNode>),
    Array(Vec<(usize, DiffTreeNode)>),
}

impl<'a> DiffTreeNode {
    pub fn get_diffs(&'a self) -> Vec<DiffEntry<'a>> {
        let mut buf = Vec::new();
        self.follow_path(&mut buf, &[]);
        buf
    }

    pub fn follow_path<'b>(
        &'a self,
        diffs: &mut Vec<DiffEntry<'a>>,
        offset: &'b [PathElement<'a>],
    ) {
        match self {
            DiffTreeNode::Null => {
                let is_map_child = offset
                    .last()
                    .map(|o| matches!(o, PathElement::Object(_)))
                    .unwrap_or_default();
                if is_map_child {
                    diffs.push(DiffEntry {
                        path: offset.to_vec(),
                        values: None,
                    });
                }
            }
            DiffTreeNode::Value(l, r) => diffs.push(DiffEntry {
                path: offset.to_vec(),
                values: Some((l, r)),
            }),
            DiffTreeNode::Node(o) => {
                for (k, v) in o {
                    let mut new_offset = offset.to_vec();
                    new_offset.push(PathElement::Object(k));
                    v.follow_path(diffs, &new_offset);
                }
            }
            DiffTreeNode::Array(v) => {
                for (l, k) in v {
                    let mut new_offset = offset.to_vec();
                    new_offset.push(PathElement::ArrayEntry(*l));
                    k.follow_path(diffs, &new_offset);
                }
            }
        }
    }
}
