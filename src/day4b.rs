
fn to_list_digits(num : i32) -> Vec<i32> {
    num.to_string()
    .chars()
    .map(|ch| ch.to_string().parse::<i32>().unwrap())
    .collect()
}

fn is_six_digit(password: i32) -> bool {
    password > 99_999 && password < 1_000_000
}


fn some_adjacent(password : i32) -> bool {
    let digits = to_list_digits(password);
    let result = digits.into_iter()
        .fold(Some((true, -1, -1)), |r, d| {
            match r {
                Some(t) =>
                    match t {
                    (part_of_group, x, y) => {
                        if x != y && (y == d) {
                            Some((false, y, d))
                        } else if x == y && y == d {
                            Some((true, y, d))
                        } else if !part_of_group && x == y && y != d {
                            None
                        } else {
                            Some((part_of_group, y, d))
                        }
                    }
                },
                _ => None
            }
        });
    match result {
        Some((part_of_group, x, y)) => !part_of_group && x == y,
        _ => true
    }
}

fn never_decrease(password : i32) -> bool {
    let digits = to_list_digits(password);
    let result = digits.into_iter()
        .fold(Some(0), |r, d| {
            match r {
                Some(x) => if x <= d { Some(d) } else {None},
                _ => None
            }
        });
    match result {
        Some(_) => true,
        _ => false
    }
}

fn valid_password(password : i32) -> bool {
    is_six_digit(password) &&
    some_adjacent(password) &&
    never_decrease(password)
}

#[allow(dead_code)]
pub fn calc() -> i32 {
    let mut counter = 0;
    for x in 147_981..691_423 {
        if valid_password(x) {
            counter = counter + 1;
        }
    } 
    counter
}

#[test]
fn test_valid_password() {
    assert_eq!(valid_password(111111), false);
    assert_eq!(valid_password(223450), false);
    assert_eq!(valid_password(123789), false);
    assert_eq!(valid_password(123799), true);
    assert_eq!(valid_password(111122), true);
}
