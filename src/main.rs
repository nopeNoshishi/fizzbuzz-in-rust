//! FizzBuzz Program
//!
//! The program repeats output based on the following rules with values up
//! to a positive integer given on the command line.
//!
//! The rules are as follows
//! - Outputs "Fizz" if multiple of fizz number
//! - Outputs "Buzz" if multiple of buzz number
//! - Outputs "FizzBuzz" if least common multiple of fizz number and buzz number
//! - Outputs the number if other than the above

use num::integer::lcm;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
struct Argument {
    /// Upper limit of integers to which the fizzbuzz rule applies
    #[structopt(short, long)]
    number: usize,

    // Multiples for which you want to output Fizz
    #[structopt(short, long)]
    fizz: usize,

    // Multiples for which you want to output Bizz
    #[structopt(short, long)]
    buzz: usize,

    /// Flag to calculate the remaining numbers in fizzbuzz
    #[structopt(short)]
    calu_falg: bool,
}

fn main() {
    let arg = Argument::from_args();
    let answer = fizzbuzz(arg.number, arg.fizz, arg.buzz);

    println!("---FizzBuzz---");
    for i in answer.clone() {
        println!("{}", i);
    }
    println!("--------------");

    if arg.calu_falg {
        println!("No FizzBuzz Number Sum: {}", calculate(answer));
    }
}

/// FizzBuzz Function
///
/// Converts numbers matching the fizzbuzz rule among 1 to given integers
/// to strings.
///
/// ## Example
/// ```
/// let vec = ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz"]
/// let test_case: Vec<String> = vec.iter().map(|&s| s.to_string()).collect();
///
/// assert_eq!(fizzbuzz(10), test_case);
/// ```
fn fizzbuzz(number: usize, fizz: usize, buzz: usize) -> Vec<String> {
    // Caluculates least common multiple
    let lcm = lcm(fizz, buzz);

    let mut answer = Vec::<String>::with_capacity(number);
    for n in 1..(number + 1) {
        if n % lcm == 0 {
            answer.push("FizzBuzz".to_string());
        } else if n % fizz == 0 {
            answer.push("Fizz".to_string());
        } else if n % buzz == 0 {
            answer.push("Buzz".to_string());
        } else {
            answer.push(format!("{}", n));
        }
    }

    answer
}

/// No FizzBuzz Calculate Function
///
/// Calculates the sum of strings that can be parsed into number
/// obtained by fizzbuzz function.
///
/// ## Example
/// ```
/// let answer = fizzbuzz(10, 3, 5);
///
/// // 1 + 2 + 4 + 7 + 8 = 22
/// assert_eq!(calculate(answer), 22);
/// ```
fn calculate(fizzbuzz_ans: Vec<String>) -> usize {
    let sum: usize = fizzbuzz_ans
        .iter()
        .filter_map(|x| x.parse::<usize>().ok())
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        let vec = [
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz", "Fizz", "22", "23", "Fizz", "Buzz",
            "26", "Fizz", "28", "29", "FizzBuzz", "31", "32", "Fizz", "34", "Buzz", "Fizz", "37",
            "38", "Fizz", "Buzz", "41", "Fizz", "43", "44", "FizzBuzz", "46", "47", "Fizz", "49",
            "Buzz", "Fizz", "52", "53", "Fizz", "Buzz", "56", "Fizz", "58", "59", "FizzBuzz", "61",
            "62", "Fizz", "64", "Buzz", "Fizz", "67", "68", "Fizz", "Buzz", "71", "Fizz", "73",
            "74", "FizzBuzz", "76", "77", "Fizz", "79", "Buzz", "Fizz", "82", "83", "Fizz", "Buzz",
            "86", "Fizz", "88", "89", "FizzBuzz", "91", "92", "Fizz", "94", "Buzz", "Fizz", "97",
            "98", "Fizz", "Buzz",
        ];
        let test_case: Vec<String> = vec.iter().map(|&s| s.to_string()).collect();

        assert_eq!(fizzbuzz(100, 3, 5), test_case);
    }

    #[test]
    fn test_calculate() {
        let vec = [
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz", "Fizz", "22", "23", "Fizz", "Buzz",
            "26", "Fizz", "28", "29", "FizzBuzz", "31", "32", "Fizz", "34", "Buzz", "Fizz", "37",
            "38", "Fizz", "Buzz", "41", "Fizz", "43", "44", "FizzBuzz", "46", "47", "Fizz", "49",
            "Buzz", "Fizz", "52", "53", "Fizz", "Buzz", "56", "Fizz", "58", "59", "FizzBuzz", "61",
            "62", "Fizz", "64", "Buzz", "Fizz", "67", "68", "Fizz", "Buzz", "71", "Fizz", "73",
            "74", "FizzBuzz", "76", "77", "Fizz", "79", "Buzz", "Fizz", "82", "83", "Fizz", "Buzz",
            "86", "Fizz", "88", "89", "FizzBuzz", "91", "92", "Fizz", "94", "Buzz", "Fizz", "97",
            "98", "Fizz", "Buzz",
        ];
        let test_case: Vec<String> = vec.iter().map(|&s| s.to_string()).collect();

        assert_eq!(calculate(test_case), 2632);
    }
}
