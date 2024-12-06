#![allow(dead_code)]

use std::ops::Add;

fn add(i: i32, j: i32) -> i32 {
    i.add(j)
}

fn bad_add(i: i32, j: i32) -> i32 {
    return i - j;
}

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0f64 {
        Ok(number.powf(0.5))
    } else {
        Err("У отрицательного вещественного числа нет квадратного корня".to_owned())
    }
}

fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error")
    } else if a < b {
        panic!("Divide result is zero")
    }

    a / b
}

#[cfg(test)]
mod tests {

    use super::*;

    type Result = std::result::Result<(), String>;

    #[test]
    fn test_add() {
        assert_eq!(add(100, 1), 101);
    }

    // #[test]
    // fn test_bad_add() {
    //     assert_eq!(bad_add(100, 1), 101);
    // }

    #[test]
    fn test_sqrt() -> Result {
        let x = 4f64;
        let expected = 2f64;
        let actual = sqrt(x)?;

        assert_eq!(expected, actual);

        Ok(())
    }

    // #[test]
    // fn test_bad_sqrt() -> Result {
    //     let x = -4f64;
    //     let expected = 2f64;
    //     let actual = sqrt(x)?;

    //     assert_eq!(expected, actual);

    //     Ok(())
    // }

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 2);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 2);
    }

    #[test]
    #[should_panic(expected = "Divide-by-zero error")]
    fn test_specific_panic_2() {
        divide_non_zero_result(1, 0);
    }
}

pub fn incr(i: i32) -> i32 {
    i + 1
}
