/*
Instructions

Given a year, report if it is a leap year.

The tricky thing here is that a leap year in the Gregorian calendar occurs:

on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400

For example, 1997 is not a leap year, but 1996 is. 1900 is not a leap year, but 2000 is.
*/

fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            }
        } else {
            return true;
        }
    }
    false
}

#[test]
fn test_early_years() {
    assert!(!is_leap_year(1));

    assert!(is_leap_year(4));

    assert!(!is_leap_year(100));

    assert!(is_leap_year(400));

    assert!(!is_leap_year(900));
}

#[test]
fn test_year_divisible_by_400_but_not_by_125_is_still_a_leap_year() {
    assert!(is_leap_year(2400));
}

#[test]
fn test_year_divisible_by_100_but_not_by_3_is_still_not_a_leap_year() {
    assert!(!is_leap_year(1900));
}
