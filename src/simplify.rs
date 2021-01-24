use crate::parsing;
use colored::*;

pub fn display_simplified(simplified: [f32; 3]) {
    print!("Reduced form:");

    if simplified[2] == 0.0 {
        if simplified[1] == 0.0 {
            println!(" {} = 0\n{}", simplified[0], "Polynomial degree: 0".blue());

            if simplified[0] == 0.0 {
                println!("{}", "Each real number is a solution.".green());
            } else {
                println!(
                    "{}",
                    "There is no solution, because the equation is impossible to solve.".yellow()
                );
            }
            std::process::exit(0);
        }

        println!(" {} + {}*X = 0", simplified[0], simplified[1]);
        println!(
            "{}\n{}",
            "Polynomial degree: 1".blue(),
            format!(
                "The solution is {}/{} (= {}).",
                simplified[0] * -1.0,
                simplified[1],
                (simplified[0] * -1.0) / simplified[1]
            )
            .green()
        );
        std::process::exit(0);
    }

    for expo in 0..3 {
        if simplified[expo] != 0.0 {
            if simplified[expo] > 0.0 && expo > 0 {
                print!(" +");
            }
            print!(" {}*X^{}", simplified[expo], expo);
        }
    }
    println!(" = 0")
}

pub fn simplify(complicated: Vec<parsing::Term>) -> [f32; 3] {
    let mut tmp: Vec<f32> = vec![0.0; 10];

    for x in complicated.iter() {
        let len = tmp.len();
        if x.exponent >= len {
            tmp.resize(x.exponent + 1, 0.0);
        }
        if x.positive == true {
            tmp[x.exponent] += x.coefficient;
        } else {
            tmp[x.exponent] -= x.coefficient;
        }
    }

    for (index, value) in tmp.iter().enumerate().rev() {
        if value.clone() != 0.0 && index > 2 {
            eprintln!(
                "{}",
                "Sorry, ComputorV1 can't solve a polynomial with a degree greater than 2.".red()
            );
            std::process::exit(1);
        }
    }

    let mut simplified: [f32; 3] = [0.0, 0.0, 0.0];
    for expo in 0..3 {
        simplified[expo] = tmp[expo];
    }
    simplified
}
