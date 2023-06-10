/*
Instructions
------------
Calculate the number of grains of wheat on a chessboard given that the number on each square doubles.

There once was a wise servant who saved the life of a prince. The king promised to pay whatever the
servant could dream up. Knowing that the king loved chess, the servant told the king he would like
to have grains of wheat. One grain on the first square of a chess board, with the number of grains
doubling on each successive square.

There are 64 squares on a chessboard (where square 1 has one grain, square 2 has two grains,
and so on).

Write code that shows:

    how many grains were on a given square,
    the total number of grains on the chessboard, and
    panics with a message of "Square must be between 1 and 64" if the value is not valid

For bonus points:
-----------------
    Optimize for speed.
    Optimize for readability.
*/

fn grain_on_square(sq: u32, total_sq: u32) -> isize {
    if !(1..=total_sq).contains(&sq) {
        panic!("Square must be between 1 and {}", total_sq);
    }
    (2isize).pow(sq - 1)
}

fn total_grains(total_sq: u32) -> isize {
    let x: isize = 2;

    1isize + (x * (x.pow(total_sq -1) - 1)) / (x - 1)
}

#[test]
fn calc_total_32(){
    let input: u32 = 32;
    let expected_output : isize = 4294967295;
    let actual_result : isize = total_grains(input);
    assert_eq!(expected_output, actual_result);
}

#[test]
fn calc_on_10() {
    let input: u32 = 10;
    let expected_output : isize = 512;
    let actual_result : isize = grain_on_square(input, 12);
    assert_eq!(expected_output, actual_result);
}