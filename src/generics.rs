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

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Add,
};

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

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self))
    }
}

pub fn traits_where() {
    let vec: Vec<i32> = vec![1, 2, 3];

    vec.print_in_option();
}

#[derive(Clone, Copy)]
struct Years(i64);

impl From<Days> for Years {
    fn from(value: Days) -> Self {
        Self(value.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 > 18
}

#[derive(Clone, Copy)]
struct Days(i64);

impl From<Years> for Days {
    fn from(value: Years) -> Self {
        Self(value.0 * 365)
    }
}

pub fn new_type_idiom() {
    let age = Years(10);
    let age_days: Days = age.into();

    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.into()));
    // println!("Old enough {}", old_enough(&age_days));
}

struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &i32, b: &i32) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// `C` содержит `A` и `B`. В свете этого, необходимость снова явно указывать `A` и
// `B` огорчает.
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

pub fn trait_problem() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Содержаться ли в контейнере {} и {}? {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2),
    );
    println!("Первое число {}", container.first());
    println!("Последнее число {}", container.last());

    println!("Разница: {}", difference(&container))
}

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

pub fn phantom_data_types() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('a', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('a', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'a',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'a',
        phantom: PhantomData,
    };

    // Ошибка времени компиляции! Типы не совпадают, так что сравнение не может быть произведено:
    // println!(
    //     "_tuple1 == _tuple2 даёт в результате: {}",
    //     _tuple1 == _tuple2
    // );

    // Ошибка времени компиляции! Типы не совпадают, так что сравнение не может быть произведено:
    // println!(
    //     "_struct1 == _struct2 даёт в результате: {}",
    //     _struct1 == _struct2
    // );
}

#[derive(Debug, Clone, Copy)]
struct Inch;

#[derive(Debug, Clone, Copy)]
struct Mm;

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, PhantomData)
    }
}

pub fn phantom_data_example() {
    let one_foot: Length<Inch> = Length(12f64, PhantomData);
    let one_meter: Length<Mm> = Length(1000f64, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("один фут + один фут = {:?} дюймов", two_feet.0);
    println!("один метр + один метр = {:?} миллиметров", two_meters.0);

    // Бессмысленные операции потерпят неудачу, как и должно быть:
    // Ошибка времени компиляции: несоответствие типов.
    // let one_feter = one_foot + one_meter;
}
