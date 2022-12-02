use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};
use clap::Parser;
use colored::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// The input file to read from
    #[arg(short, long)]
    input: String,

    /// The output file to write to
    #[arg(short, long)]
    output: String,

    /// The string to search for
    #[arg(short, long)]
    search: String,

    /// The string to replace with
    #[arg(short, long)]
    replace: String,

    ///case sensitive search
    #[arg(short, long)]
    case_sensitive: bool,
}

fn has_match(line: &str, search: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        line.contains(search)
    } else {
        line.to_lowercase().contains(&search.to_lowercase())
    }
}

fn replace_match(line: &str, search: &str, replace: &str, case_sensitive: bool) -> String {
    if case_sensitive {
        line.replace(search, replace)
    } else {
        line.to_lowercase().replace(&search.to_lowercase(), replace)
    }
}

fn main() -> io::Result<()> {
    use std::time::Instant;
    let now = Instant::now();

    let args = Args::parse();

    let mut line_count = 0;
    let mut change_count = 0;

    let write_filename = args.output;
    let read_filename = args.input;
    let search = args.search;
    let replace = args.replace;

    let file = File::open(&read_filename)?;
    let reader = BufReader::new(file);
    
    match File::create(&write_filename) {
        Ok(mut file) => {
            for line in reader.lines() {
                let line = line?;
                if has_match(&line, &search, args.case_sensitive) {
                    let line = replace_match(&line, &search, &replace, args.case_sensitive);
                    writeln!(file, "{}", line)?;
                    change_count += 1;
                }
                else {
                    writeln!(file, "{}", line)?;
                }
                line_count += 1;
            }
            let elapsed = now.elapsed().as_secs_f32().to_string();
            let final_count = line_count.to_string();
            let final_change = change_count.to_string();
            println!("Replaced {} with {} on {} out of {} lines \n in {} seconds", search.color("red"), replace.color("blue"), final_change.color("red"), final_count.color("green"), elapsed.color("green"));
            println!("File written to {}", &write_filename.color("green"));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}