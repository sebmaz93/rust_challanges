use std::collections::HashMap;

fn _unique(items: Vec<u32>) -> Vec<u32> {
    let mut hashmap: HashMap<u32, u32> = HashMap::new();

    // items.iter().for_each(|&item| hashmap.insert(item, 1));
    hashmap.extend(items.into_iter().map(|item| (item, 1)));

    let mut keys = hashmap.keys().cloned().collect::<Vec<_>>();
    keys.sort();
    keys

    // short way using built-in methods
    /*
    keys.sort()
    keys.dedup()
    keys
    */
}

#[test]
fn sorted_items() {
    let input = vec![1, 1, 3, 4, 6, 8, 9, 9];
    let expected_output: Vec<u32> = vec![1, 3, 4, 6, 8, 9];
    let actual_output = _unique(input);
    assert_eq!(expected_output, actual_output);
}

#[test]
fn unsorted_items() {
    let input: Vec<u32> = vec![3, 1, 5, 2, 5, 6, 2, 1];
    let expected_output: Vec<u32> = vec![1, 2, 3, 5, 6];
    let actual_output = _unique(input);
    assert_eq!(expected_output, actual_output)
}
