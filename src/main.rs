use failure::{Context, Error};
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashSet,
    io::{self, BufRead},
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    about = "Any line coming in from stdin that is also present in the given file will be passed to stdout"
)]
struct Opt {
    file: PathBuf,

    #[structopt(
        short = "n",
        long = "negate",
        help = "If set, then only those lines in stdin that are *not* in the given file are passed to stdout"
    )]
    negate: bool,
}

fn read_file(path: &Path) -> Result<Vec<String>, Error> {
    let contents = std::fs::read(path)?;
    let contents = String::from_utf8(contents)?;
    Ok(contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(String::from)
        .collect())
}

fn construct_regex(string_to_match: &[String]) -> Result<Regex, Error> {
    let expression = string_to_match
        .iter()
        .map(|x| regex::escape(x))
        .intersperse("|".to_string())
        .collect::<String>();

    Regex::new(&expression).map_err(Error::from)
}

fn main() {
    let args = Opt::from_args();
    let lines = read_file(&args.file).expect("Failed to read input file");
    let regex = construct_regex(&lines).expect("Failed to construct regex");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read from stdin");
        if regex.is_match(&line) ^ args.negate {
            println!("{}", line);
        }
    }
}
