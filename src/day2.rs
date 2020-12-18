use nom::bytes::complete::tag;
use nom::character::complete::{anychar, space0};
use nom::character::streaming::digit1;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::IResult;
use nom::sequence::{separated_pair, terminated, tuple};

#[derive(Debug)]
struct PasswordPolicy {
    letter: char,
    min_occurrence: usize,
    max_occurrence: usize,
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn occurrence(s: &str) -> Res<&str, (&str, &str)> {
    terminated(separated_pair(digit1, opt(tag("-")), digit1), space0)(s)
}

fn letter(s: &str) -> Res<&str, char> {
    terminated(anychar, tag(": "))(s)
}

fn parse(s: &str) -> Res<&str, PasswordPolicy> {
    tuple((
        occurrence,
        letter,
    ))(s).map(|(next_input, output)| {
        let ((min, max), letter, ) = output;
        (next_input, PasswordPolicy {
            letter,
            min_occurrence: min.parse().unwrap(),
            max_occurrence: max.parse().unwrap(),
        })
    })
}

fn is_valid_password_part_1(policy: PasswordPolicy, password: &str) -> bool {
    let t = password.chars();
    let len = t.filter(|&c| c == policy.letter).collect::<Vec<char>>().len();

    len >= policy.min_occurrence && len <= policy.max_occurrence
}

fn is_valid_password_part_2(policy: PasswordPolicy, password: &str) -> bool {
    let t = password.chars();
    let first = t.clone().nth(policy.min_occurrence - 1) == Some(policy.letter);
    let second = t.clone().nth(policy.max_occurrence - 1) == Some(policy.letter);

    first ^ second
}

fn solve(input: &Vec<String>, fun: fn(PasswordPolicy, &str) -> bool) -> i32 {
    let mut res = 0;

    for line in input {
        let (password, policy) = parse(line.as_str()).unwrap();

        if fun(policy, password) {
            res += 1;
        }
    }

    res
}

pub fn solution(input: Vec<String>) -> [i32; 2] {
    [solve(&input, is_valid_password_part_1), solve(&input, is_valid_password_part_2)]
}