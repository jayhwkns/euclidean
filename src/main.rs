//! An implementation of the Euclidean greatest common divisor algorithm that
//! prints each step.
use std::env;

enum Mode {
    Default,
    Extended,
}

/// Usage: `./euclidean <options> <a> <b>` translates to gcd(a, b)
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut mode = Mode::Default;
    let num_options = args.len() - 3;
    for i in 0..num_options {
        match args[i + 1].as_str() {
            "-e" | "--extended" => {
                mode = Mode::Extended;
            }
            _ => {}
        }
    }
    let a: i64 = args[1 + num_options].parse().unwrap();
    let b: i64 = args[2 + num_options].parse().unwrap();

    match mode {
        Mode::Default => {
            let res = gcd(a, b);

            println!("gcd({a}, {b}) = {res}");
        }
        Mode::Extended => {
            let (g, u, v) = extended_euclidean(a, b);

            println!("(g, u, v) = ({g}, {u}, {v})");
        }
    }
}

/// Recursive implementation of euclidean gcd algorithm
fn gcd(a: i64, b: i64) -> i64 {
    let quotient = a / b;
    let remainder = a % b;

    println!("{a} = {b} * {quotient} + {remainder}");

    if remainder == 0 {
        return b;
    }

    gcd(b, remainder)
}

fn extended_euclidean(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut u, mut g, mut x, mut y): (i64, i64, i64, i64) = (1, a, 0, b);
    loop {
        println!("u = {u}, g = {g}, x = {x}, y = {y}");
        if y == 0 {
            let v = (g - a * u) / b;
            return (g, u, v);
        }
        let q = g / y;
        let t = g % y;
        let s = u - q * x;
        println!("q = {q}, t = {t}, s = {s}");
        u = x;
        g = y;
        x = s;
        y = t;
    }
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn textbook_works() {
        let result = gcd(2024, 748);
        assert_eq!(result, 44);
    }

    #[test]
    fn textbook_works_negative() {
        let result = gcd(-2024, -748);
        assert_eq!(result, -44);
    }
}
