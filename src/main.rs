extern crate pest;
#[macro_use]
extern crate pest_derive;

use colored::*;

use structopt::StructOpt;
/// A basic polynomial solver
#[derive(StructOpt, Debug)]
struct Cli {
    /// The equation to be solved
    input: String,

    /// Set precision for square-root computation
    /// (must be between 0.00001 and 1)
    #[structopt(short = "p", long = "precision", default_value = "0.01")]
    precision: f32,

    /// Display more computation details
    #[structopt(short, long)]
    verbose: bool,
}

mod compute;
mod parsing;
mod simplify;

fn main() {
    let args = Cli::from_args();

    if args.precision < 0.00001 || args.precision > 1.0 {
        eprintln!(
            "{}",
            "The precision value must be between 0.00001 and 1.".red()
        );
        std::process::exit(1);
    }

    let poly = args.input;

    let complicated = parsing::parsing(poly);
    let simplified = simplify::simplify(complicated);
    simplify::display_simplified(simplified);

    if args.verbose {
        println!(
            "{}",
            format!(
                "Delta: {}²-4({}×{}) = {}",
                simplified[1],
                simplified[2],
                simplified[0],
                compute::compute_delta(simplified)
            )
            .magenta()
        );
    }

    compute::solver(
        simplified,
        compute::compute_delta(simplified),
        args.precision,
        args.verbose,
    );
}
