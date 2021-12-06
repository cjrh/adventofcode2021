fn main() {
    let puzzle1_input = std::fs::read_to_string(
        // "input01a_test.txt",
        "input01a.txt",
    ).unwrap();
    // count_increases(&puzzle1_input);
    count_increases_window(&puzzle1_input);
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
