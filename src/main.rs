use std::{fs, io};

mod day1;
mod day2;
mod day3;
mod day22;

fn open_file(filename: &str) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(filename)?;

    let t = contents.lines().map(|s| s.to_string()).collect::<Vec<String>>();
    Ok(t)
}

fn main() -> io::Result<()> {
    println!("Day1: {:?}", day1::solution(open_file("./inputs/day1.dat")?));
    println!("Day2: {:?}", day2::solution(open_file("./inputs/day2.dat")?));
    println!("Day3: {:?}", day3::solution(open_file("./inputs/day3.dat")?));
    println!("Day22: {:?}", day22::solution(open_file("./inputs/day22.dat")?));

    Ok(())
}
