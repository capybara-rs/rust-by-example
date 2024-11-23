#![allow(dead_code)]

use rand::Rng;

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise())
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Self { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

pub fn traits() {
    let mut dolly: Sheep = Animal::new("Dolly");
    let mut polly = Sheep::new("Polly");

    dolly.talk();
    polly.talk();

    dolly.shear();
    polly.shear();

    dolly.talk();
    polly.talk();
}

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(f64);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

pub fn derive() {
    let _one_second = Seconds(1i32);

    // Ошибка: `Seconds` не может быть напечатана; не реализован типаж `Debug`
    // println!("Одна секунда выглядит как: {:?}", _one_second);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Ошибка: `Seconds` нельзя сравнить; не реализован типаж `PartialEq`
    // let _this_is_true = (_one_second == _one_second);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    let foot = Inches(12f64);

    let meter = Centimeters(100f64);

    let cmp = if foot.to_centimeters() < meter {
        "Меньше"
    } else {
        "Больше"
    };

    println!("Один фут {} одного метра", cmp);
}

struct Tiger {}
struct Lion {}

trait Cat {
    fn meow(&self) -> &'static str;
}

impl Cat for Tiger {
    fn meow(&self) -> &'static str {
        "mrrrrr"
    }
}

impl Cat for Lion {
    fn meow(&self) -> &'static str {
        "ROOOOOOOOAR"
    }
}

fn animal(random_number: f64) -> Box<dyn Cat> {
    if random_number > 0.5 {
        Box::new(Tiger {})
    } else {
        Box::new(Lion {})
    }
}

pub fn dyn_traits() {
    let mut rng = rand::rng();

    let number = rng.random_range(0f64..1f64);

    println!("cat says {}", animal(number).meow());
}

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;
#[derive(Debug)]
struct BarFoo;

use std::ops::Add;

impl Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _: Bar) -> Self::Output {
        FooBar
    }
}

impl Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _: Foo) -> Self::Output {
        BarFoo
    }
}

pub fn operator_overload() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Сбросили {}", self.name);
    }
}

pub fn drop_trait() {
    let _a = Droppable { name: "a" };

    {
        let _b = Droppable { name: "b" };

        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Выходим из блока Б");
        }

        println!("Вышли из блока Б");

        println!("Выходим из блока А")
    }

    println!("Вышли из блока А");

    drop(_a);

    println!("Конец главного блока")
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

impl Fibonacci {
    fn start() -> Self {
        Self { curr: 0, next: 1 }
    }
}

pub fn iterators() {
    // `0..3` это `Iterator`, который генерирует : 0, 1, и 2.
    let mut sequence = 0..3;

    println!("Четыре подряд вызова `next`на 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` работает через `Iterator` пока тот не вернет `None`.
    // каждое значение `Some` распаковывается  и привязывается к переменной (здесь это `i`).
    println!("Итерирование по 0..3 используя `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // Метод `take(n)` уменьшает `Iterator` до его первых `n` членов.
    println!("Первые четыре члена последовательности Фибоначчи: ");

    for element in Fibonacci::start().take(4) {
        println!("> {}", element);
    }

    // Метод `skip(n)` сокращает `Iterator`, отбрасывая его первые `n` членов.
    println!("Следующие четыре члена последовательности Фибоначчи: ");
    for element in Fibonacci::start().skip(4).take(4) {
        println!("> {}", element);
    }

    let array = [1u32, 3, 3, 7];

    // Метод `iter` превращает `Iterator` в массив/срез.
    println!("Итерирование по массиву {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}

use std::iter;
use std::vec::IntoIter;

// Эта функция объединяет два `Vec<i32>` и возвращает итератор.
// Посмотрите какой получается сложный тип возвращаемого значения!
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// Это та же самая функция, но в возвращаемом типе использует нотацию `impl Trait`.
// Посмотрите как он упростился!
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

pub fn impl_trait() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];

    let mut v3 = combine_vecs(v1, v2);

    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    assert_eq!(Some(1), v3.next());
    println!("готово");
}

// Вернём функцию, которая добавляет `y` ко входному значению
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

pub fn clone_trait() {
    let unit = Unit;

    let copied_unit = unit;

    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(3));
    println!("original pair: {:?}", pair);

    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Ошибка! Переменная `pair` потеряла свои ресурсы
    // println!("original: {:?}", pair);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    let cloned_pair = moved_pair.clone();

    println!("moved pair: {:?}", moved_pair);

    drop(moved_pair);

    // Ошибка! `moved_pair` была удалена
    // println!("copy: {:?}", moved_pair);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Результат, полученный из .clone(), все ещё можно использовать!
    println!("clone: {:?}", cloned_pair);
}

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &impl CompSciStudent) -> String {
    format!(
        "Меня зовут {} и я посещаю {}. Мое имя в git {}",
        student.name(),
        student.university(),
        student.git_username(),
    )
}

struct Dima;

impl Person for Dima {
    fn name(&self) -> String {
        String::from("Дима")
    }
}

impl Student for Dima {
    fn university(&self) -> String {
        String::from("ШАРАГА")
    }
}

impl Programmer for Dima {
    fn fav_language(&self) -> String {
        String::from("Rust")
    }
}

impl CompSciStudent for Dima {
    fn git_username(&self) -> String {
        String::from("amidman")
    }
}

pub fn super_traits() {
    println!("{}", comp_sci_student_greeting(&Dima))
}

trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    user_name: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.user_name.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

pub fn trait_polymorphizm() {
    let form = Form {
        user_name: "rustacean".to_owned(),
        age: 20,
    };

    // Если вы раскомментируете эту строку, вы получите ошибку, которая говорит
    // "multiple `get` found". Потому что это, в конце концов, несколько методов
    // с именем `get`.
    // println!("{}", form.get());

    let user_name = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), user_name);

    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(20, age);
}
