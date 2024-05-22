use std::collections::HashMap;

use serde_json::Value;

use crate::enums::{DiffEntry, PathElement};

#[derive(Debug, PartialEq)]
pub enum KeyNode {
    Nil,
    Value(Value, Value),
    Node(HashMap<String, KeyNode>),
    Array(Vec<(usize, KeyNode)>),
}

impl KeyNode {
    pub fn get_diffs(&self) -> Vec<DiffEntry> {
        let mut buf = Vec::new();
        self.follow_path(&mut buf, &[]);
        buf
    }

    pub fn follow_path(&self, diffs: &mut Vec<DiffEntry>, offset: &[PathElement]) {
        match self {
            KeyNode::Nil => {
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
            KeyNode::Value(l, r) => diffs.push(DiffEntry {
                path: offset.to_vec(),
                values: Some((l.to_string(), r.to_string())),
            }),
            KeyNode::Node(o) => {
                for (k, v) in o {
                    let mut new_offset = offset.to_vec();
                    new_offset.push(PathElement::Object(k.clone()));
                    v.follow_path(diffs, &new_offset);
                }
            }
            KeyNode::Array(v) => {
                for (l, k) in v {
                    let mut new_offset = offset.to_vec();
                    new_offset.push(PathElement::ArrayEntry(*l));
                    k.follow_path(diffs, &new_offset);
                }
            }
        }
    }
}
