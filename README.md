# json-diff-ng

[![Crates.io](https://img.shields.io/crates/d/json_diff_ng?style=flat)](https://crates.io/crates/json_diff_ng)
[![Documentation](https://docs.rs/json_diff_ng/badge.svg)](https://docs.rs/json_diff_ng)
![CI](https://github.com/ChrisRega/json-diff/actions/workflows/rust.yml/badge.svg?branch=master "CI")
[![Coverage Status](https://coveralls.io/repos/github/ChrisRega/json-diff/badge.svg?branch=master)](https://coveralls.io/github/ChrisRega/json-diff?branch=master)
[![License](https://img.shields.io/github/license/ChrisRega/json-diff)](LICENSE)

## Contributors:

<a href="https://github.com/ChrisRega/json-diff/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=ChrisRega/json-diff"  alt="Contributors"/>
</a>

## Library

json_diff_ng can be used to get diffs of json-serializable structures in rust.

### Usage example

```rust
use json_diff::compare_strs;
let data1 = r#"["a",{"c": ["d","f"] },"b"]"#;
let data2 = r#"["b",{"c": ["e","d"] },"a"]"#;
let diffs = compare_strs(data1, data2, true, & []).unwrap();
assert!(!diffs.is_empty());
let diffs = diffs.unequal_values.get_diffs();
assert_eq!(diffs.len(), 1);
assert_eq!(
    diffs.first().unwrap().to_string(),
    r#".[0].c.[1].("f" != "e")"#
);
```

See [docs.rs](https://docs.rs/json_diff_ng) for more details.

## CLI

json-diff is a command line utility to compare two jsons.

Input can be fed as inline strings or through files.  
For readability, output is neatly differentiated into three categories: keys with different values, and keys not present
in either of the objects.  
Only missing or unequal keys are printed in output to reduce the verbosity.

Usage Example:

`$ json_diff file source1.json source2.json`  
`$ json_diff direct '{...}' '{...}'`

Option:

file   :   read input from json files  
direct   :   read input from command line

### Installation

`$ cargo install json_diff_ng`

