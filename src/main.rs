extern crate structopt;

use structopt::StructOpt;

extern crate colored;
use colored::*;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meowz!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::from_args(); //
    let message = options.message;
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }
    let eye = if options.dead { "x" } else { "o" }; // [1]

    println!("{}", message.bright_yellow().underline()
    .on_purple());
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {eye} {eye} )", eye=eye.red().bold()); // [2]
    println!("    =( I )=");
}