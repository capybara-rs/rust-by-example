#![allow(dead_code)]

pub fn unimplemented() {
    unimplemented!("Эта функция ещё не готова");
}

fn drink(beverage: &str) {
    if beverage == "лимонад" {
        panic!("Ненавижу лимонад!")
    }

    println!("Немного свежего {} это всё что мне нужно", beverage);
}

pub fn panic_example() {
    drink("апельсиного сока");
    drink("лимонад");
}

fn give_adult(drink: Option<&str>) {
    match drink {
        Some("лимонад") => println!("Фи! Слишком сладко."),
        Some(drink) => println!("Я возьму {}? Хорошо.", drink),
        None => println!("Умираю от жажды!"),
    }
}

fn drink_optional(drink: Option<&str>) {
    let inside = drink.expect("пустое значение недопустимо");

    if inside == "лимонад" {
        panic!("Ненавижу лимонад!");
    }

    println!("Обожаю {}", inside);
}

pub fn option_example() {
    let water = Some("вода");
    let lemonade = Some("лимонад");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("кофе");
    let nothing = None;

    drink_optional(coffee);
    drink_optional(nothing);
}

fn next_bithday(current_age: Option<u16>) -> Option<String> {
    let next_age: u16 = current_age?;

    Some(format!("В следующем году мне будет {}", next_age))
}

struct Job<'a> {
    phone_number: Option<&'a PhoneNumber>,
}

impl<'a> Job<'a> {
    fn new(phone_number: &'a PhoneNumber) -> Self {
        Self {
            phone_number: Some(phone_number),
        }
    }
}

struct PhoneNumber {
    area_code: Option<u8>,
    number: u64,
}

impl PhoneNumber {
    fn new(area_code: u8, number: u64) -> Self {
        Self {
            number,
            area_code: Some(area_code),
        }
    }
}

struct Person<'a> {
    job: Option<&'a Job<'a>>,
}

impl<'a> Person<'a> {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?
            .phone_number?
            .area_code
    }

    fn new(job: &'a Job) -> Self {
        Self { job: Some(job) }
    }
}

pub fn unwrapping_option() {
    let area_code = 7u8;
    let number = 8005553535u64;
    let phone_number = PhoneNumber::new(area_code, number);

    let job = Job::new(&phone_number);
    let person = Person::new(&job);

    assert_eq!(person.work_phone_area_code(), Some(area_code));
}

enum Fruit {
    Apple,
    Orange,
    Mango,
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fruit::Apple => write!(f, "Яблоко"),
            Fruit::Orange => write!(f, "Апельсин"),
            Fruit::Mango => write!(f, "Манго"),
        }
    }
}

struct Peeled(Fruit);
struct Chopped(Fruit);
struct Cooked(Fruit);

fn peel(fruit: Option<Fruit>) -> Option<Peeled> {
    let fruit = fruit?;

    Some(Peeled(fruit))
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(fruit)) => Some(Chopped(fruit)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(fruit: Option<Fruit>) -> Option<Cooked> {
    fruit
        .map(|f: Fruit| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(cooked: Option<Cooked>) {
    match cooked {
        Some(Cooked(fruit)) => println!("Я очень люблю {}!", fruit),
        None => println!("Хочу фруктов"),
    }
}

pub fn option_chain_map_example() {
    use Fruit::*;

    let apple = Some(Apple);
    let orange = Some(Orange);
    let mango = Some(Mango);

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_orange = cook(chop(peel(orange)));
    let cooked_mango = process(mango);

    eat(cooked_apple);
    eat(cooked_orange);
    eat(cooked_mango);
    eat(None);
}

enum Food {
    CordonBlue,
    Steak,
    Sushi,
}

impl std::fmt::Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Food::CordonBlue => write!(f, "Кордон Блю"),
            Food::Steak => write!(f, "Стейк"),
            Food::Sushi => write!(f, "Суши"),
        }
    }
}

enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Day::Monday => write!(f, "Понедельник"),
            Day::Tuesday => write!(f, "Вторник"),
            Day::Wednesday => write!(f, "Среду"),
        }
    }
}

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBlue => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        Some(food) => have_ingredients(food),
        None => None,
    }
}

fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
    // have_recipe(food).map(have_ingredients)
    // попробуй раскоментировать эту ^^^ строчку
}

fn eat_food(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Кчау! В {} мы будем есть {}", day, food),
        None => println!("О нет, Мы не будем есть в {}", day),
    }
}

pub fn optional_flat_map() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBlue, Food::Steak, Food::Sushi);

    eat_food(cordon_bleu, Day::Monday);
    eat_food(steak, Day::Tuesday);
    eat_food(sushi, Day::Wednesday);
}

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print_result<T>(result: Result<T, ParseIntError>)
where
    T: std::fmt::Display,
{
    match result {
        Ok(number) => println!("n равно {}", number),
        Err(e) => println!("Ошибка {}", e),
    }
}

fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str
        .parse::<i32>()
        .and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
}

type MultiplyRes<T> = Result<T, ParseIntError>;

fn multiply_v3(first_number_str: &str, second_number_str: &str) -> MultiplyRes<i32> {
    first_number_str
        .parse::<i32>()
        .and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
}

pub fn result() {
    let twenty = multiply("10", "2");
    print_result(twenty);

    let twenty = multiply("ten", "two");
    print_result(twenty);

    let twenty = multiply_v2("10", "2");
    print_result(twenty);

    let twenty = multiply_v2("ten", "two");
    print_result(twenty);

    let twenty = multiply_v3("10", "2");
    print_result(twenty);

    let twenty = multiply_v3("ten", "two");
    print_result(twenty);
}

pub enum MultiplyError {
    Io(std::io::Error),
    ParseInt(ParseIntError),
}

impl From<std::io::Error> for MultiplyError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ParseIntError> for MultiplyError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl std::fmt::Display for MultiplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiplyError::Io(error) => write!(f, "received io error, {}", error),
            MultiplyError::ParseInt(parse_int_error) => {
                write!(f, "received parse int error, {}", parse_int_error)
            }
        }
    }
}

type MultiplyResult<T> = Result<T, MultiplyError>;

fn multiply_input() -> MultiplyResult<i32> {
    let input = std::io::stdin();

    let mut first_line = String::new();
    let mut second_line = String::new();

    input.read_line(&mut first_line)?;
    input.read_line(&mut second_line)?;

    let first_number = first_line
        .trim_end_matches("\n")
        .parse::<i32>()?;

    let second_number = second_line
        .trim_end_matches("\n")
        .parse::<i32>()?;

    Ok(first_number * second_number)
}

pub fn result_early_return() {
    match multiply_input() {
        Ok(number) => println!("{number}"),
        Err(e) => println!("{e}"),
    }
}

fn double_first(v: &[&str]) -> i32 {
    let first = v.first().unwrap();
    let first = first.parse::<i32>().unwrap();

    first * 2
}

fn double_first_v2(v: &[&str]) -> Option<Result<i32, ParseIntError>> {
    v.first()
        .map(|&first: &&str| {
            first
                .parse::<i32>()
                .map(|i: i32| i * 2)
        })
}

pub fn unwrap_result() {
    let numbers = vec!["42", "2", "1"];
    let _empty: Vec<&str> = vec![];
    let strings = vec!["tofu"];

    println!("Первое удвоенное {}", double_first(&numbers));

    println!("Первое удвоенное {:?}", double_first_v2(&_empty));
    // Ошибка 1: входной вектор пустой

    println!("Первое удвоенное {:?}", double_first_v2(&strings));
    // Ошибка 2: элемент не может быть преобразован в число
}

fn double_first_v3(v: &[&str]) -> Result<Option<i32>, ParseIntError> {
    let opt: Option<Result<i32, ParseIntError>> = v
        .first()
        .map(|&first: &&str| {
            first
                .parse::<i32>()
                .map(|first: i32| first * 2)
        });

    opt.map_or(
        Ok(None),
        |r: Result<i32, ParseIntError>| -> Result<Option<i32>, ParseIntError> {
            r.map(|i: i32| Some(i))
        },
    )
}

pub fn convert_result_to_option() {
    let none: Option<i32> = None;
    let some: Option<i32> = Some(100);
    let er = none.map(Some);
    let ok = some.map(Some);

    println!("none map to res: {:?}", er);
    println!("some map to res: {:?}", ok);
}

pub mod result {
    use std::fmt::Display;

    pub struct DoubleError;

    impl Display for DoubleError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "неверный первый элемент")
        }
    }

    type Result<T> = std::result::Result<T, DoubleError>;

    fn print_result<T>(r: Result<T>)
    where
        T: Display,
    {
        match r {
            Ok(value) => println!("Получено значение {}", value),
            Err(e) => println!("Получена ошибка {}", e),
        }
    }

    fn double_first(v: &[&str]) -> Result<i32> {
        v.first()
            .ok_or(DoubleError)
            .and_then(|&first: &&str| {
                first
                    .parse::<i32>()
                    .map_err(|_| DoubleError)
                    .map(|i| i * 2)
            })
    }

    pub fn custom_result() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print_result(double_first(&numbers));
        print_result(double_first(&empty));
        print_result(double_first(&strings));
    }
}

pub mod error_wrapping {
    use core::num;
    use std::{error, fmt, result};

    type Result<T> = result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug)]
    struct EmptyVecError;

    impl fmt::Display for EmptyVecError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "неверный первый элемент")
        }
    }

    impl error::Error for EmptyVecError {}

    fn print_result<T: fmt::Display>(r: Result<T>) {
        match r {
            Ok(value) => println!("Получено значение: {value}",),
            Err(e) => println!("Ошибка {e}"),
        }
    }

    fn double_first(v: &[&str]) -> Result<i32> {
        v.first()
            .ok_or_else(|| EmptyVecError.into())
            .and_then(|first: &&str| {
                first
                    .parse::<i32>()
                    .map_err(|_: num::ParseIntError| EmptyVecError.into())
                    .map(|first: i32| first * 2)
            })
    }

    fn double_first_v2(v: &[&str]) -> Result<i32> {
        let first = v
            .first()
            .ok_or(EmptyVecError)?;

        let first: i32 = first.parse()?;

        Ok(first * 2)
    }
}

pub mod wrap_error {

    use std::{error, fmt, num, result};

    type Result<T> = result::Result<T, DoubleError>;

    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        Parse(num::ParseIntError),
    }

    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                DoubleError::EmptyVec => write!(f, "empty vector"),
                DoubleError::Parse(parse_int_error) => {
                    write!(f, "parse int error, {}", parse_int_error)
                }
            }
        }
    }

    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match self {
                DoubleError::EmptyVec => None,
                DoubleError::Parse(parse_int_error) => Some(parse_int_error),
            }
        }
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("Первое удвоение {}", n),
            Err(e) => println!("Ошибка: {}", e),
        }
    }

    fn double_first(v: &[&str]) -> Result<i32> {
        let first_item = v
            .first()
            .ok_or(DoubleError::EmptyVec)?;

        let first: i32 = first_item
            .parse()
            .map_err(DoubleError::Parse)?;

        Ok(first * 2)
    }

    pub fn example() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(&numbers));
        print(double_first(&empty));
        print(double_first(&strings));
    }
}

pub mod iterate {
    use std::{collections::BTreeMap, num::ParseIntError};

    pub fn problem() {
        let strings = vec!["apple", "1", "12"];
        let results: Vec<Result<i32, ParseIntError>> = strings
            .into_iter()
            .map(|s: &str| s.parse())
            .collect();
        println!("Результаты {:?}", results);
    }

    pub fn filter_err() {
        let strings = vec!["apple", "1", "12"];
        let numbers: Vec<i32> = strings
            .into_iter()
            .map(|s: &str| s.parse())
            .filter_map(Result::ok)
            .collect();
        println!("Результаты {:?}", numbers);
    }

    pub fn interrupt_iteration() {
        let strings = vec!["1", "apple", "12"];
        let result: Result<Vec<i32>, ParseIntError> = strings
            .into_iter()
            .map(|s: &str| s.parse())
            .collect();
        println!("Результаты {:?}", result)
    }

    pub fn partitioning() {
        let strings = vec!["1", "apple", "12"];
        let (results, errors): (
            Vec<Result<i32, ParseIntError>>,
            Vec<Result<i32, ParseIntError>>,
        ) = strings
            .into_iter()
            .map(|s: &str| s.parse())
            .partition(|r: &Result<i32, ParseIntError>| -> bool { r.is_ok() });

        let numbers: Vec<i32> = results
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        let errors: Vec<ParseIntError> = errors
            .into_iter()
            .filter_map(Result::err)
            .collect();

        println!("Числа: {:?}", numbers);
        println!("Ошибки: {:?}", errors);
    }
}
