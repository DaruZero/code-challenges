use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
    process::exit,
};

use utf8_chars::BufReadCharsExt;

enum Action {
    Help,
    Bytes,
    Lines,
    Words,
    Chars,
}

fn main() {
    let args: Vec<_> = args().collect();

    let action = match validate_args(&args) {
        Ok(action) => match action {
            Action::Help => {
                print_help();
                exit(0)
            }
            _ => action,
        },
        Err(why) => {
            println!("Wrong usage: {}", why);
            print_help();
            exit(1)
        }
    };

    let path = Path::new(args[2].as_str());
    let display = path.display();

    let result = match action {
        Action::Help => {
            print_help();
            exit(0)
        }
        Action::Bytes => count_bytes(path),
        Action::Lines => count_lines(path),
        Action::Words => count_words(path),
        Action::Chars => count_chars(path),
    };

    println!("{result}\t{display}")
}

fn validate_args(args: &[String]) -> Result<Action, &'static str> {
    match args.len() {
        1 => Ok(Action::Help),
        2 => {
            if args[1] != "-h" && args[1] != "--help" {
                Err("Missing argument")
            } else {
                Ok(Action::Help)
            }
        }
        3 => match args[1].as_str() {
            "-c" | "--bytes" => Ok(Action::Bytes),
            "-l" | "--lines" => Ok(Action::Lines),
            "-w" | "--words" => Ok(Action::Words),
            "-m" | "--chars" => Ok(Action::Chars),
            _ => Err("Unrecognized option"),
        },
        _ => Err("Too many arguments"),
    }
}

fn print_help() {
    println!(
        "Usage: wc [OPTION] [FILE]
Print newline, word, and byte counts for each FILE

The options below may be used
  -c, --bytes            print the byte count
  -l, --lines            print the line count
  -w, --words            print the word count
  -m, --chars            print the character count"
    );
}

fn count_bytes(path: &Path) -> usize {
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut buf = Vec::new();
    if let Err(why) = file.read_to_end(&mut buf) {
        panic!("couldn't read {}: {}", display, why)
    }

    buf.len()
}

fn count_lines(path: &Path) -> usize {
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    reader.lines().count()
}

fn count_words(path: &Path) -> usize {
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut buf: String = Default::default();
    if let Err(why) = file.read_to_string(&mut buf) {
        panic!("couldn't read {}: {}", display, why)
    }

    buf.split_whitespace().count()
}

fn count_chars(path: &Path) -> usize {
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);

    reader.chars().count()
}
