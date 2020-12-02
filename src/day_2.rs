pub fn solution() {
    let input = parse_input();

    println!("Part 1 solution: = {}", part_1(&input));
    println!("Part 2 solution: = {}", part_2(&input));
}
fn part_1(input: &Vec<(PasswordRule, String)>) -> usize {
    input
        .iter()
        .filter(|(rule, password)| rule.validate(password))
        .count()
}

fn part_2(input: &Vec<(PasswordRule, String)>) -> usize {
    input
        .iter()
        .filter(|(rule, password)| rule.validate_part_2(password))
        .count()
}
fn parse_input() -> Vec<(PasswordRule, String)> {
    std::fs::read_to_string("./input2.txt")
        .expect("Input file couldn't be read")
        .split('\n')
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(ln, s)| {
            let s_vec: Vec<&str> = s.split(':').collect();
            let (rule, password) = s_vec
                .split_first()
                .expect(&format!("Empty line {}: {}", ln, s));
            (
                PasswordRule::from_str(rule).expect(&format!("No rule on line {}: {}", ln, s)),
                password
                    .first()
                    .expect(&format!("No password on line {}: {}", ln, s))
                    .trim()
                    .to_string(),
            )
        })
        .collect::<Vec<(PasswordRule, String)>>()
}
#[derive(Debug)]
struct PasswordRule {
    char: char,
    min: usize,
    max: usize,
}
impl PasswordRule {
    fn from_str(s: &str) -> Option<Self> {
        let s = s.split('-').flat_map(|s| s.split(' ')).collect::<Vec<_>>();
        if let [min, max, char] = s[..=2] {
            Some(Self {
                char: char.chars().next()?,
                min: min.parse().ok()?,
                max: max.parse().ok()?,
            })
        } else {
            None
        }
    }

    fn validate(&self, password: &str) -> bool {
        if password.matches(self.char).count() <= self.max
            && password.matches(self.char).count() >= self.min
        {
            true
        } else {
            false
        }
    }

    fn validate_part_2(&self, password: &str) -> bool {
        (password.chars().nth(self.min - 1).unwrap() == self.char)
            ^ (password.chars().nth(self.max - 1).unwrap() == self.char)
    }
}
