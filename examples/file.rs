use std::env::args;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use geosteiner::{euclidean_steiner_tree, rectilinear_steiner_tree};

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    ParseError(std::num::ParseFloatError),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::ParseError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "error reading file: {}", e),
            Error::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

fn read_from_file(path_buf: PathBuf) -> Result<Vec<[f64; 2]>> {
    let file = std::fs::File::open(path_buf).unwrap();
    let reader = BufReader::new(file);
    let mut points = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break
        }
        let mut iter = line.split_whitespace();
        let x = iter.next().unwrap().parse::<f64>()?;
        let y = iter.next().unwrap().parse::<f64>()?;
        points.push([x, y]);
    }
    Ok(points)
}

#[derive(Debug, Default)]
enum TreeType {
    #[default]
    Euclidean,
    Rectilinear,
}

impl TreeType {
    fn from_str(s: &str) -> Option<Self> {
        if s.is_empty() {
            None
        } else if "euclidean".starts_with(s) {
            Some(Self::Euclidean)
        } else if "rectilinear".starts_with(s) {
            Some(Self::Rectilinear)
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
struct Args {
    input: Option<PathBuf>,
    typ: TreeType,
}

const HELP: &str = "\
Options:
    -h, --help      \tPrint this help message
    -t, --type TYPE \tTree type: euclidean or rectilinear [default: euclidean]\
";

fn print_help(name: &str) {
    println!("Usage: {} INPUT", name);
    println!("{}", HELP);
}

fn argparse(args: &[&str]) -> Option<Args> {
    let name = args[0];
    let mut remainder = &args[1..];
    let mut parsed = Args::default();
    loop {
        match remainder {
            ["-h" | "--help", ..] => {
                print_help(name);
                return None
            },
            ["-t" | "--type", typ, rem @ ..] => {
                if let Some(typ) = TreeType::from_str(typ) {
                    parsed.typ = typ;
                    remainder = rem;
                } else {
                    println!("Invalid tree type: {}", typ);
                    return None
                }
            },
            [input, rem @ ..] => {
                if parsed.input.is_some() {
                    println!("Multiple input files specified");
                    return None
                }
                parsed.input = Some(PathBuf::from(input));
                remainder = rem;
            },
            [] => break
        }
    }
    if parsed.input.is_none() {
        println!("No input file specified");
        return None
    }
    Some(parsed)
}

fn main() {
    let args: Vec<_> = args().collect();
    let argref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let args = argparse(&argref).unwrap_or_else(|| {
        print_help(&args[0]);
        std::process::exit(1);
    });
    let points = read_from_file(args.input.unwrap()).unwrap_or_else(|e| {
        eprintln!("Error reading file: {}", e);
        std::process::exit(1);
    });
    let tree = match args.typ {
        TreeType::Euclidean => euclidean_steiner_tree(&points),
        TreeType::Rectilinear => rectilinear_steiner_tree(&points),
    };
    println!("steiner = {:?}", tree.steiner_points);
    println!("edges = {:?}", tree.edges);
}
