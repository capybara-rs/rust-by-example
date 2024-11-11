#![allow(dead_code)]

struct A;

struct Single(A);

struct SingleGen<T>(T);

impl<T> SingleGen<T> {
    fn value(&self) -> &T {
        return &self.0;
    }
}

pub fn structs() {
    let _s = Single(A);
    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6i32);
    let _char = SingleGen('1');
}

fn reg_fn(_s: Single) {}

fn gen_spec_t(_s: SingleGen<A>) {}

fn gen_spec_i32(_s: SingleGen<i32>) {}

fn gen_generic<T>(_s: SingleGen<T>) {}

pub fn functions() {
    reg_fn(Single(A));
    gen_spec_t(SingleGen(A));
    gen_spec_i32(SingleGen(0i32));

    gen_generic::<f64>(SingleGen(0f64));

    gen_generic(SingleGen('1'));
}

struct CharContainer(char);

impl CharContainer {
    fn value(&self) -> &char {
        &self.0
    }
}

pub fn implementations() {
    let x = SingleGen('c');
    let y = CharContainer('c');

    println!("{}, {}", x.value(), y.value())
}

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _t: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _t: T) {}
}

pub fn traits() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // empty.double_drop(null);
    // null.double_drop(empty);
}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {
    "красный"
}

fn blue<T: Blue>(_: &T) -> &'static str {
    "синий"
}

pub fn empty_traits() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // `red()` не будет работать для blue_jay, ни наоборот,
    // из-за ограничений по трейту.
    println!("Кардинал {} птица", red(&cardinal));
    println!("Голубая сойка {} птица", blue(&blue_jay));
    // println!("Индюк {} птица", red(&_turkey));
    // ^ TODO: Попробуйте раскомментировать эту строку.
}

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

pub fn trait_limits() {
    let string = "Hello World!";
    let array = ["Hello", " ", "World!"];
    let vec = vec!["Hello", " ", "World!"];

    compare_prints(&string);
    // compare_prints(&array);
    compare_types(&array, &vec);
}
