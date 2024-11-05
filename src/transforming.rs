#![allow(dead_code, unused)]

use std::fmt;

#[derive(Debug, PartialEq)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

pub fn from_into() {
    let i = 100i32;

    let number_from = Number::from(i);
    let number_into: Number = i.into();

    assert_eq!(number_from, number_into);
}

#[derive(Debug, PartialEq)]
struct OddNumber(i32);

impl TryFrom<i32> for OddNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 1 {
            Ok(OddNumber(value))
        } else {
            Err(())
        }
    }
}

pub fn try_from_into() {
    assert_eq!(OddNumber::try_from(100), Err(()));
    assert_eq!(OddNumber::try_from(101), Ok(OddNumber(101)));

    let result: Result<OddNumber, ()> = 101.try_into();
    assert_eq!(result, Ok(OddNumber(101)));
    let result: Result<OddNumber, ()> = 202.try_into();
    assert_eq!(result, Err(()));
}

struct Circle {
    radius: i32,
}

impl Circle {
    fn diameter(&self) -> i32 {
        &self.radius * 2
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Круг радиусом {} и диаметром {}",
            self.radius,
            self.diameter()
        )
    }
}

pub fn string_transforming() {
    let radius = 2;
    let circle = Circle { radius };
    println!("{}", circle.to_string());
    println!("{}", Circle { radius: 3 }.to_string());
}
