//! An implementation of the Euclidean greatest common divisor algorithm that
//! prints each step.
use std::env;

/// Usage: `./euclidean <a> <b>` translates to gcd(a, b)
fn main() {
    let args: Vec<String> = env::args().collect();

    let a: i64 = args[1].parse().unwrap();
    let b: i64 = args[2].parse().unwrap();

    let res = gcd(a, b);

    println!("gcd({a}, {b}) = {res}");
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
