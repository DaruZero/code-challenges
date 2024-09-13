use std::{env::args, process::exit};

const FLAG_HELP: &str = "-h";
const FLAG_BYTE: &str = "-c";

fn main() {
    // if no args or FLAG_HELP arg, print help and exit
    if args().len() == 1 && args().nth(2).expect("Cannot read args") == FLAG_HELP {
        print_help();
        exit(0);
    }

    if args().len() > 3 {
        println!("Too many arguments");
        print_help();
        exit(1)
    }

    match args().nth(2) {
        FLAG_BYTE => {
            count_bytes();
        }
        _ => {
            println!("Unrecognized argument");
            print_help();
            exit(1)
        }
    }
}

fn print_help() {
    println!(
        "Usage: wc [OPTION] [FILE]
Print byte counts for each FILE

The options below may be used
  -c, --bytes            print the byte counts"
    );
}

fn count_bytes() {}
