fn parse_row(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c| match c {
            '#' => 1,
            _ => 0,
        })
        .collect()
}

fn traverse_map(
    grid: Vec<Vec<u32>>,
    step: (i32, i32),
    position: (i32, i32),
    trees_hit: u32,
) -> u32 {
    if position.1 >= grid.len() as i32 {
        return trees_hit;
    }

    let row = grid.get(position.1 as usize).unwrap();

    let cell = row.get(position.0 as usize).unwrap();
    traverse_map(
        grid.clone(),
        step,
        (
            (position.0 + step.0) % row.len() as i32,
            position.1 + step.1,
        ),
        trees_hit + cell,
    )
}

fn solve_part1(rows: &[Vec<u32>]) -> u32 {
    traverse_map(rows.to_owned(), (3, 1), (0, 0), 0)
}

fn solve_part2(rows: &[Vec<u32>]) -> u32 {
    let hits: Vec<u32> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|step| traverse_map(rows.to_owned(), *step, (0, 0), 0))
        .collect();

    hits.iter().product()
}

pub fn solution(input: Vec<String>) -> [u32; 2] {
    let rows: Vec<Vec<u32>> = input.iter().map(|s| parse_row(s)).collect();

    [solve_part1(&rows), solve_part2(&rows)]
}
