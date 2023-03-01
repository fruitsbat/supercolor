use clap::{Parser, ValueEnum};
use colored::*;
use pad::PadStr;
use rand::{distributions::WeightedIndex, prelude::*, Rng};
use std::{fs, include_str};
use strum::IntoEnumIterator;

mod builtins;

// test the example with `cargo run --example most_simple`
fn main() {
    let args = Args::parse();

    let files = match &args.path {
        Some(p) => read_path(p),
        None => vec![],
    };
    let builtin_or_file = [DataType::Builtin, DataType::File];
    let weights = [builtins::Builtins::iter().count(), files.len()];
    let dist = WeightedIndex::new(&weights).expect("failed to parse weighted index list!");

    let mut rng = rand::thread_rng();
    let builtin_or_file = &builtin_or_file[dist.sample(&mut rng)];
    let data = match builtin_or_file {
        DataType::Builtin => builtins::Builtins::iter()
            .choose(&mut rng)
            .unwrap_or(builtins::Builtins::Catpeek)
            .get(),
        DataType::File => files.choose(&mut rng).unwrap(),
    };
    print(data, args);
}

enum DataType {
    Builtin,
    File,
}

/// give number of rainbow color n this returns string in that color
fn rainbowify<S: Into<String>>(input: S, number: u64) -> ColoredString {
    match number.rem_euclid(6) {
        0 => input.into().red(),
        1 => input.into().green(),
        2 => input.into().yellow(),
        3 => input.into().blue(),
        4 => input.into().magenta(),
        5 => input.into().cyan(),
        // waht the fuck
        _ => input.into().red(),
    }
}

fn read_path(path: &str) -> Vec<String> {
    match fs::read_dir(path) {
        Ok(fs) => fs
            .map(|file| file.unwrap().path().as_os_str().to_str().unwrap().into())
            .collect::<Vec<String>>(),
        _ => vec![path.into()],
    }
}

fn print<S: Into<String>>(input: S, args: Args) {
    let rows = args.rows;
    let columns = args.columns;
    let input: String = input.into();
    let mut longest: usize = 0;
    let offset = args
        .color_offset
        .unwrap_or(rand::thread_rng().gen_range(0..5));

    for line in input.lines() {
        if line.chars().count() > longest {
            longest = line.chars().count();
        }
    }

    let mut linecount = 0;
    for row in 0..rows {
        for line in input.lines() {
            linecount += 1;
            for column in 0..columns {
                let color = (offset
                    + match args.color_mode {
                        ColorMode::Solid => column + row,
                        ColorMode::Uniform => 0,
                        ColorMode::Rainbow => linecount,
                    }) as u64;
                if column == columns - 1 {
                    println!("{}", rainbowify(line, color));
                } else {
                    print!("{}", rainbowify(line.pad_to_width(longest), color))
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ColorMode {
    Solid,
    Rainbow,
    Uniform,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// how many times it will be printed horizontally
    #[arg(short, long, default_value_t = 1)]
    rows: u8,

    /// how many times it will be printed vertically
    #[arg(short, long, default_value_t = 6)]
    columns: u8,

    /// which mode to use
    #[arg(long, value_enum, default_value = "solid")]
    color_mode: ColorMode,

    /**
    color to start with, uses random if not set
    0 = red
    1 = green
    2 = yellow
    3 = blue
    4 = magenta
    5 = cyan
    */
    #[arg(long)]
    color_offset: Option<u8>,

    #[arg(short, long)]
    path: Option<String>,
}
