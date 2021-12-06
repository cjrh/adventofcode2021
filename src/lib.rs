pub fn xxx(x: i32) -> i32 {
    x + 1
}

pub fn get_input_data(name: &str) -> String {
    std::fs::read_to_string(name).unwrap()
}
