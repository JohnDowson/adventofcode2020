pub fn solution() {
    let input = parse_input();

    println!("Part 1 solution: = {}", part_1(&input).unwrap());
    println!("Part 2 solution: = {}", part_2(&input).unwrap());
}
fn part_1(input: &Vec<i32>) -> Option<i32> {
    for i in input {
        let target = 2020 - i;
        if let Some(x) = input.iter().find(|&&item| item == target) {
            return Some(i * x);
        }
    }
    None
}

fn part_2(input: &Vec<i32>) -> Option<i32> {
    for i1 in input {
        for i2 in input {
            let target = 2020 - (i1 + i2);
            if let Some(x) = input.iter().find(|&&item| item == target) {
                return Some(i1 * i2 * x);
            }
        }
    }
    None
}
fn parse_input() -> Vec<i32> {
    std::fs::read_to_string("./input_1.txt")
        .expect("Input file couldn't be read")
        .split('\n')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>()
}
