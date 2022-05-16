fn main() {}

fn get_age(age: &str) -> u32 {
    age.chars()
        .find(|x| x.is_digit(10))
        .unwrap()
        .to_digit(10)
        .unwrap()
}

#[test]
fn basic_tests() {
    assert_eq!(get_age("2 years old"), 2);
    assert_eq!(get_age("4 years old"), 4);
    assert_eq!(get_age("5 years old"), 5);
    assert_eq!(get_age("7 years old"), 7);
}
