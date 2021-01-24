use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct PolyParser;

use colored::*;

pub struct Term {
    pub positive: bool,
    pub coefficient: f32,
    pub exponent: usize,
}

pub fn parsing(equation: String) -> Vec<Term> {
    std::panic::set_hook(Box::new(|_| {
        //Manage panic caused by the parse function
        eprintln!("{}", "Parsing failed. Please check user input.".red());
    }));

    let parse = PolyParser::parse(Rule::equation, &equation)
        .expect("Parse failed")
        .next()
        .unwrap();

    let mut complicated: Vec<Term> = Vec::new();
    let mut lh_poly: bool = false;
    let mut first_term: bool = true;
    let mut cnt: usize = 0;

    for digit in parse.into_inner().flatten() {
        match digit.as_rule() {
            Rule::polynomial => {
                lh_poly ^= true; //Switch the value of the boolean
                if lh_poly == false {
                    cnt += 1;
                    first_term = true;
                }
                complicated.insert(
                    cnt,
                    Term {
                        positive: lh_poly,
                        coefficient: 1.0,
                        exponent: 0,
                    },
                );
            }

            Rule::sign => {
                if first_term == false {
                    cnt += 1;
                    complicated.insert(
                        cnt,
                        Term {
                            positive: true,
                            coefficient: 1.0,
                            exponent: 0,
                        },
                    );
                }

                let mut sign: bool = true;

                if digit.as_str() == "-" {
                    sign = false;
                }
                if lh_poly == false {
                    sign ^= true;
                }
                complicated[cnt].positive = sign;
                first_term = false;
            }

            Rule::number => {
                complicated[cnt].coefficient = digit.as_str().trim().parse::<f32>().unwrap();
                first_term = false;
            }

            Rule::variable => {
                complicated[cnt].exponent = 1;
                first_term = false;
            }

            Rule::exponent => {
                complicated[cnt].exponent = digit
                    .as_str()
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
            }

            _ => (),
        }
    }
    complicated
}
