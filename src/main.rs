mod day_1;
mod day_2;
fn main() {
    match std::env::args().nth(1) {
        Some(day) => match day.parse().unwrap() {
            1 => day_1::solution(),
            2 => day_2::solution(),
            _ => {}
        },
        None => {
            day_1::solution();
            day_2::solution()
        }
    }
}
