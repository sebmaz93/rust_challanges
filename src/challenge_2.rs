use std::collections::HashMap;

fn _unique(items: Vec<i32>) -> Vec<i32> {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    // items.iter().for_each(|&item| hashmap.insert(item, 1));
    hashmap.extend(items.into_iter().map(|item| (item, 1)));

    let mut keys = hashmap.keys().cloned().collect::<Vec<_>>();
    keys.sort();
    keys
}

#[test]
fn sorted_items() {
    let input = vec![1, 1, 3, 4, 6, 8, 9, 9];
    let expected_output = vec![1, 3, 4, 6, 8, 9];
    let actual_output = _unique(input);
    assert_eq!(expected_output, actual_output);
}
