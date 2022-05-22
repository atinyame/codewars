fn main() {}

fn get_count(string: &str) -> usize {
    string.chars().filter(|&x| "aeiou".contains(x)).count()
}

// fn get_count(string: &str) -> usize {
//     string
//         .matches(|x| match x {
//             'a' | 'e' | 'i' | 'o' | 'u' => true,
//             _ => false,
//         })
//         .count()
// }

#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}
