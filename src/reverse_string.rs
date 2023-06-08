fn reverse(input: &str) -> String {
    let mut temp = input.chars().collect::<Vec<_>>();
    temp.reverse();
    temp.iter().collect::<String>()
}

#[test]
fn test_string() {
    let input = "you did it!";
    let expected_output = "!ti did uoy";
    let actual_output = reverse(input);
    assert_eq!(expected_output, actual_output)
}
