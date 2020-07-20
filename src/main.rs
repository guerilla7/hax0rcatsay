extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meowz!")]
    /// What does the cat say?
    message: String // [1]
}

fn main() {
    let options = Options::from_args(); // [2]
    let message = options.message;
    println!("{}", message);
    // ...print the cat
}