use colored::*;

fn solver_verbose(simplified: [f32; 3], delta: f32, x1: f32, x2: f32) {
    println!(
        "{}",
        format!(
            "Here are the exact values:\n\tx1 = ({} + √{}) / {}\n\tx2 = ({} - √{}) / {}",
            simplified[1] * -1.0,
            delta,
            2.0 * simplified[2],
            simplified[1] * -1.0,
            delta,
            2.0 * simplified[2]
        )
        .magenta()
    );

    let gcd = compute_gcd(simplified[1], 2.0 * simplified[2]);
    if compute_absolute_value(gcd) > 1.0 {
        let simplified_sqrt = compute_simplified_sqrt(1.0, delta);
        println!(
            "{}",
            format!(
                "These values can be simplified as:\n\tx1 = ({} + {}) / {}\n\tx2 = ({} - {}) / {}",
                // Simplification of x1
                simplified[1] / gcd * -1.0,
                if simplified_sqrt.0 != 1.0 {
                    format!("{}√{}", simplified_sqrt.0 / gcd, simplified_sqrt.1)
                } else {
                    format!("√{}", delta)
                },
                2.0 * simplified[2] / gcd,
                // Simplification of x2
                simplified[1] / gcd * -1.0,
                if simplified_sqrt.0 != 1.0 {
                    format!("{}√{}", simplified_sqrt.0 / gcd, simplified_sqrt.1)
                } else {
                    format!("√{}", delta)
                },
                2.0 * simplified[2] / gcd,
            )
            .magenta()
        );
    }

    println!(
        "{}",
        format!(
            "The polynomial can be factorised as: {}(x-{})(x+{})",
            simplified[2],
            if x1 < 0.0 { x1 * -1.0 } else { x1 },
            if x2 < 0.0 { x2 * -1.0 } else { x2 }
        )
        .magenta()
    );
}

pub fn solver(simplified: [f32; 3], delta: f32, precision: f32, verbose: bool) {
    let root = compute_square_root(compute_absolute_value(delta), precision);

    println!("{}", "Polynomial degree: 2".blue());

    if delta > 0.0 {
        let x1 = ((simplified[1] * -1.0) + root) / (2.0 * simplified[2]);
        let x2 = ((simplified[1] * -1.0) - root) / (2.0 * simplified[2]);
        println!(
            "{}",
            format!("The discriminant is strictly positive, so there's 2 solutions in {}:\n\tx1 = {}\n\tx2 = {}", "R".bold(), x1, x2).green()
        );
        if verbose {
            solver_verbose(simplified, delta, x1, x2);
        }
    } else if delta < 0.0 {
        println!(
            "{}",
            format!(
                "The discriminant is strictly negative, so there's 2 solutions in {}:\n\tx1 = {} - {}i\n\tx2 = {} + {}i",
                "C".bold(),
                -1.0 * simplified[1] / (2.0 * simplified[2]),
                root / (2.0 * simplified[2]),
                -1.0 * simplified[1] / (2.0 * simplified[2]),
                root / (2.0 * simplified[2])
            )
            .green()
        )
    } else {
        println!(
            "{}",
            format!(
                "The discrimant is zero, so there's only 1 solution: x = {}.",
                (simplified[1] * -1.0) / (2.0 * simplified[2])
            )
            .green()
        )
    }
}

fn compute_absolute_value(nb: f32) -> f32 {
    if nb < 0.0 {
        return nb * -1.0;
    }
    return nb;
}

fn compute_square_root(nb: f32, precision: f32) -> f32 {
    let mut mean = (nb + 1.0) / 2.0;
    while compute_absolute_value(mean * mean - nb) > precision {
        let estimate = nb / mean;
        mean = (mean + estimate) / 2.0;
    }
    return mean;
}

pub fn compute_delta(simplified: [f32; 3]) -> f32 {
    (simplified[1] * simplified[1]) - 4.0 * (simplified[0] * simplified[2])
}

fn compute_gcd(a: f32, b: f32) -> f32 {
    if b != 0.0 {
        compute_gcd(b, a % b)
    } else {
        a
    }
}

fn compute_simplified_sqrt(mut outside: f32, mut inside: f32) -> (f32, f32) {
    let mut d = 2.0;

    while d * d <= inside {
        if inside % (d * d) == 0.0 {
            inside = inside / (d * d);
            outside = outside * d;
        } else {
            d += 1.0;
        }
    }
    (outside, inside)
}
