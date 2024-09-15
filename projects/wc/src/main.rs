use std::{env::args, fs::File, io::Read, path::Path, process::exit};

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
        let bytes = count_bytes(args[2].as_str());
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

fn count_bytes(path_to_file: &str) -> usize {
    let path = Path::new(path_to_file);
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
