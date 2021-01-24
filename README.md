# README
This a [School 42](https://www.42.fr/) project. The PDF of the subject is [here](https://cdn.intra.42.fr/pdf/pdf/13223/en.subject.pdf). 

This was my first time coding in Rust, and it was a fun experience.  
I know using a parser generator (in this case, [pest](https://pest.rs/)) is a bit overkill and regexes would have done the job fine. But I wanted to try writing [my own formal grammar](/src/grammar.pest), and I find the end-result a lot more legible than a bunch of regular expressions.

## Computor V1
This project aims to make you code a simple equation solving program. It will take polynomial equations into account. These equations will only require exponents. No complex function. The program will have to display its solution(s).

It should manage all polynomials of the first and second degree, as long as they're formatted properly (you can find some examples under the `tests` rule in the Makefile). Thanks to [structop](https://github.com/TeXitoi/structopt), there's a nifty `--help` flag.

The subject requires that no math lib function (except for subtraction, division, addition and multiplication of real numbers) can be used. So I had to code my own function for square root computation, using the [Babylonian method](https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method).

### How to install and launch
0. Make sure you have [Rust installed on your machine](https://doc.rust-lang.org/stable/book/ch01-01-installation.html).
1. `git clone` the project
2. Compile with `make`. If your system can't find `cargo` even after the Rust installation went properly, you may need to `source $HOME/.cargo/env`.
3. Run with `./computor <equation you want to be solved>`.

### Bonus
- Manage free form entries (i.e. `X` is the same as `X^0`, powers can be in any order)
- The `-v` flag displays more details (such as the discriminant) and also computes:
    - the irreducible fraction
    - the simplified square root
    - the factorization of the polynomial
- Colorized output
- The `-p` flag allows changing the precision of the square root computation (default value is 0.01)