extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meowz!")]
    /// What does the cat say?
    message: String, // [1]

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::from_args(); //
    let message = options.message;

    let eye = if options.dead { "x" } else { "o" }; // [1]

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {eye} {eye} )", eye=eye); // [2]
    println!("    =( I )=");
}