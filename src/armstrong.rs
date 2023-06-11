/*
Instructions
------------
An Armstrong number is a number that is the sum of its own digits each raised to the power of 
the number of digits.

For example:
    9 is an Armstrong number, because 9 = 9^1 = 9
    10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
    153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
    154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190

Write some code to determine whether a number is an Armstrong number.
*/

fn is_armstrong(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|n| n.to_string().parse().expect("provide a valid number!"))
        .collect();

    let length = digits.len();
    let sum: u64 = digits.iter().fold(0, |acc, &x| acc + u64::from(x.pow(length as u32)));
    sum == u64::from(num)
}

#[test]
fn test_three_digit_armstrong_number() {
    assert!(is_armstrong(153))
}

#[test]
fn test_ten_digit_non_armstrong_number() {
    assert!(!is_armstrong(3_999_999_999));
}

#[test]
fn test_properly_handles_overflow() {
    assert!(!is_armstrong(4_106_098_957));
}
