fn main() {
    let puzzle1_input = adventofcode2021::get_input_data("input01a.txt");
    count_increases_window(&puzzle1_input);
}

fn count_increases_window(input: &str) -> i32 {
    let v = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut increases = 0;
    let mut prev = 1e8 as i32;

    for i in 0..v.len() - 2 {
        let sum = v.get(i).unwrap()
            + v.get(i+1).unwrap()
            + v.get(i+2).unwrap();
        if sum > prev {
            increases += 1;
        }
        prev = sum;
    }
    println!("increases: {}", increases);
    0
}

