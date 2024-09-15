use std::{env::args, process::exit};

fn main() {
    let args: Vec<_> = args().collect();

    // if no args or `-h`, print help and exit
    if args.len() == 1 || (&args[1] == "-h" || &args[1] == "--help") {
        print_help();
        exit(0);
    }

    if args.len() > 3 {
        println!("Too many arguments");
        print_help();
        exit(1)
    }

    if args[1] == "-c" || args[1] == "--bytes" {
        count_bytes()
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

fn count_bytes() {
    panic!("not implemented");
}
