#[derive(Debug)]
struct Arguments {
    filename: String
}

use regex::Regex;

use text_colorizer::*;

use std::env;

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("{} wrong number of arguments: expected 1, got {}.",
            "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        filename: args[0].clone(),
    }
}

use std::fs;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2014-01-01"));

    println!("Hello, world!");

    let args = parse_args();
    println!("{:?}", args);

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to read from file");
            std::process::exit(1);
        }
    };    

    println!("{}", data);


    // Stores the iterator of lines of the file in lines variable.
    let lines = read_lines(args.filename);
    // Iterate over the lines of the file, and in this case print them.
    let s7g_re = Regex::new(r"7g").unwrap();
    for line in lines {
        let line_str = line.as_ref().unwrap();
        if s7g_re.is_match(&line_str) {
            println!("{}", line.unwrap());
        }
    }
}
