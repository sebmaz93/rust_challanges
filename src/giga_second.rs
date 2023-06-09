use time::{ PrimitiveDateTime as DateTime, Duration, Date, Time, Month };
use time_macros::{ date, time };

fn add_giga_second(date: DateTime) -> DateTime {
    date + Duration::seconds(1_000_000_000)
}

#[test]
fn test_date() {
    let input: DateTime = DateTime::new(date!(2015 - 01 - 24), time!(22:00:00));
    let expected_output = DateTime::new(date!(2046 - 10 - 02), time!(23:46:40));
    let actual_output = add_giga_second(input);
    assert_eq!(expected_output, actual_output);
}
