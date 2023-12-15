fn main() {
    let inputs = include_str!("../../inputs/day1-1");
    let lines = inputs.lines();
    let sum: u32 = lines.map(num_by_line).sum();
    println!("{:?}", sum)
}

fn num_by_line(line: &str) -> u32 {
    let mut numbers = line.chars().filter(|c| c.is_ascii_digit());
    let first_num = numbers.next().unwrap();
    if let Some(number) = numbers.last() {
        let combined = first_num.to_string() + &number.to_string();
        combined.parse().unwrap()
    } else {
        let combined = first_num.to_string() + &first_num.to_string();
        combined.parse().unwrap()
    }
}
