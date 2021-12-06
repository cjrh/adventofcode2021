fn main() {
    let puzzle1_input = adventofcode2021::get_input_data("input01a.txt");
    count_increases(&puzzle1_input);
}

fn count_increases(input: &str) -> i32 {
    let v = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut increases = 0;
    let mut prev = 1e8 as i32;
    for x in &v {
        if x > &prev {
            increases += 1;
        };
        prev = *x;
    }
    println!("increases: {}", increases);
    0
}

