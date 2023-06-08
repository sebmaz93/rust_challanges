fn _median(mut items: Vec<usize>) -> Option<f64> {
    if items.is_empty() {
        return None;
    }

    // using usize to make sorting easier since sort is not available by default on f64 type :(
    items.sort();
    let n_elems = items.len();
    let middle_elem = n_elems / 2;
    let items_is_even = n_elems % 2 == 0;

    let med = if items_is_even {
        ((items[middle_elem] as f64) + (items[middle_elem - 1] as f64)) / 2.0
    } else {
        items[middle_elem] as f64
    };

    Some(med)
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output: Option<f64> = None;
    let actual_output = _median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_list() {
    let input = vec![5, 2, 8, 1];
    let expected_output: Option<f64> = Some(3.5);
    let actual_output = _median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn odd_list() {
    let input = vec![7, 3, 9];
    let expected_output: Option<f64> = Some(7.0);
    let actual_output = _median(input);
    assert_eq!(actual_output, expected_output)
}
