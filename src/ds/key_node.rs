use std::collections::HashMap;

use serde_json::Value;

use crate::enums::{DiffEntry, PathElement};

#[derive(Debug, PartialEq)]
pub enum TreeNode {
    Null,
    Value(Value, Value),
    Node(HashMap<String, TreeNode>),
    Array(Vec<(usize, TreeNode)>),
}

impl TreeNode {
    pub fn get_diffs(&self) -> Vec<DiffEntry> {
        let mut buf = Vec::new();
        self.follow_path(&mut buf, &[]);
        buf
    }

    pub fn follow_path(&self, diffs: &mut Vec<DiffEntry>, offset: &[PathElement]) {
        match self {
            TreeNode::Null => {
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
            TreeNode::Value(l, r) => diffs.push(DiffEntry {
                path: offset.to_vec(),
                values: Some((l.to_string(), r.to_string())),
            }),
            TreeNode::Node(o) => {
                for (k, v) in o {
                    let mut new_offset = offset.to_vec();
                    new_offset.push(PathElement::Object(k.clone()));
                    v.follow_path(diffs, &new_offset);
                }
            }
            TreeNode::Array(v) => {
                for (l, k) in v {
                    let mut new_offset = offset.to_vec();
                    new_offset.push(PathElement::ArrayEntry(*l));
                    k.follow_path(diffs, &new_offset);
                }
            }
        }
    }
}
