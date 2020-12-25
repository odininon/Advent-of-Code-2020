pub fn solution(input: Vec<String>) -> [i32; 2] {
    let numbers = input
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    [part1(&numbers), part2(&numbers)]
}

fn part1(numbers: &[i32]) -> i32 {
    for number1 in numbers {
        for number2 in numbers {
            if number1 + number2 == 2020 {
                return number1 * number2;
            }
        }
    }

    -1
}

fn part2(numbers: &[i32]) -> i32 {
    for number1 in numbers {
        for number2 in numbers {
            for number3 in numbers {
                if number1 + number2 + number3 == 2020 {
                    return number1 * number2 * number3;
                }
            }
        }
    }

    -1
}
