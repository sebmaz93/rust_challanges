/*
Instructions
------------
Your task is to convert a number into a string that contains raindrop sounds corresponding to
certain potential factors. A factor is a number that evenly divides into another number, leaving
no remainder. The simplest way to test if a one number is a factor of another is to use the modulo
operation.

The rules of raindrops are that if a given number:
--------------------------------------------------
has 3 as a factor, add 'Pling' to the result.
has 5 as a factor, add 'Plang' to the result.
has 7 as a factor, add 'Plong' to the result.
does not have any of 3, 5, or 7 as a factor, the result should be the digits of the number.

Examples
--------
28 has 7 as a factor, but not 3 or 5, so the result would be "Plong".
30 has both 3 and 5 as factors, but not 7, so the result would be "PlingPlang".
34 is not factored by 3, 5, or 7, so the result would be "34".
*/

fn make_raindrop_sounds(n: i32) -> String {
    let mut sound = String::new();
    if n % 3 == 0 {
        sound.push_str("Pling");
    }
    if n % 5 == 0 {
        sound.push_str("Plang");
    }
    if n % 7 == 0 {
        sound.push_str("Plong");
    }

    if sound.is_empty() {
        n.to_string()
    } else {
        sound
    }
}

#[test]
fn test_1() {
    assert_eq!("Pling", make_raindrop_sounds(9))
}

#[test]
fn test_2() {
    assert_eq!("Plang", make_raindrop_sounds(10))
}

#[test]
fn test_3() {
    assert_eq!("Plong", make_raindrop_sounds(14))
}

#[test]
fn test_4() {
    assert_eq!("PlingPlangPlong", make_raindrop_sounds(105))
}

#[test]
fn test_5() {
    assert_eq!("34", make_raindrop_sounds(34))
}
