fn median(mut items: Vec<usize>) -> Option<usize> {
    if items.is_empty() {
        return None;
    }

    items.sort();
    let n_elems = items.len();
    let middle_elem = n_elems / 2;
    let items_is_even = n_elems % 2 == 0;

    let med = if items_is_even {
        (items[middle_elem] + items[middle_elem - 1]) / 2
    } else {
        items[middle_elem]
    };

    Some(med)
}
