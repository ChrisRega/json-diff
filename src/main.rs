use clap::Parser;
use clap::Subcommand;

use json_diff_ng::{compare_strs, Mismatch, Result};

#[derive(Subcommand, Clone)]
/// Input selection
enum Mode {
    /// File input
    #[clap(short_flag = 'f')]
    File { file_1: String, file_2: String },
    /// Read from CLI
    #[clap(short_flag = 'd')]
    Direct { json_1: String, json_2: String },
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Mode,

    #[clap(short, long)]
    /// deep-sort arrays before comparing
    sort_arrays: bool,

    #[clap(short, long)]
    /// Exclude a given list of keys by regex.
    exclude_keys: Option<Vec<String>>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Getting input");
    let (json_1, json_2) = match args.cmd {
        Mode::Direct { json_2, json_1 } => (json_1, json_2),
        Mode::File { file_2, file_1 } => {
            let d1 = vg_errortools::fat_io_wrap_std(file_1, &std::fs::read_to_string)?;
            let d2 = vg_errortools::fat_io_wrap_std(file_2, &std::fs::read_to_string)?;
            (d1, d2)
        }
    };
    println!("Evaluation exclusion regex list");
    let exclusion_keys = args
        .exclude_keys
        .as_ref()
        .map(|v| {
            v.iter()
                .map(|k| regex::Regex::new(k).map_err(|e| e.into()))
                .collect::<Result<Vec<regex::Regex>>>()
                .unwrap_or_default()
        })
        .unwrap_or_default();
    println!("Comparing");
    let mismatch = compare_strs(&json_1, &json_2, args.sort_arrays, &exclusion_keys)?;
    println!("Printing results");
    let comparison_result = check_diffs(mismatch)?;
    if !comparison_result {
        std::process::exit(1);
    }
    Ok(())
}

pub fn check_diffs(result: Mismatch) -> Result<bool> {
    let mismatches = result.all_diffs();
    let is_good = mismatches.is_empty();
    for (d_type, key) in mismatches {
        println!("{d_type}: {key}");
    }
    Ok(is_good)
}
