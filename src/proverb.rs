fn make_proverb(list: &[&str]) -> String {
    let len = list.len();

    if len == 0 {
        return String::from("");
    }

    let mut res = String::new();

    fn phrase(x: &str, y: Option<&str>) -> String {
        if let Some(y_str) = y {
            return format!("For want of a {} the {} was lost.", x, y_str);
        }
        format!("And all for the want of a {}.", x)
    }

    for (idx, i) in list.iter().enumerate() {
        if idx + 1 == len {
            res.push_str(&phrase(list[0], None));
        } else {
            res.push_str(&phrase(i, Some(list[idx + 1])));
            res.push('\n');
        }
    }

    res

    //     Another solution
    /*
    match list.first() {
        None => String::new(),
        Some(word) => list.windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(once(format!("And all for the want of a {}.", word)))
            .collect(),
    }
    */
}

#[test]
fn test_two_pieces() {
    let input = vec!["nail", "shoe"];
    let expected = vec![
        "For want of a nail the shoe was lost.",
        "And all for the want of a nail."
    ].join("\n");
    assert_eq!(make_proverb(&input), expected);
}

#[test]
fn test_three_pieces() {
    let input = vec!["nail", "shoe", "horse"];
    let expected = vec![
        "For want of a nail the shoe was lost.",
        "For want of a shoe the horse was lost.",
        "And all for the want of a nail."
    ].join("\n");
    assert_eq!(make_proverb(&input), expected);
}

#[test]
fn test_one_piece() {
    let input = vec!["nail"];
    let expected = String::from("And all for the want of a nail.");
    assert_eq!(make_proverb(&input), expected);
}

#[test]
fn test_zero_pieces() {
    let input: Vec<&str> = vec![];
    let expected = String::new();
    assert_eq!(make_proverb(&input), expected);
}
