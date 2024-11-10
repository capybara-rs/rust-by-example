#![allow(dead_code)]

use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0f64, y: 0f64 }
    }
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

struct Rectangle {
    left_top: Point,
    right_bottom: Point,
}

struct Perimeter(f64);

impl From<f64> for Perimeter {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl fmt::Display for Perimeter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Периметр прямоугольника: {}", self.0)
    }
}

struct Area(f64);

impl From<f64> for Area {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Площадь прямоуголька: {}", self.0)
    }
}

impl Rectangle {
    fn area(&self) -> Area {
        let Point {
            x: left_top_x,
            y: left_top_y,
        } = self.left_top;
        let Point {
            x: right_bottom_x,
            y: right_bottom_y,
        } = self.right_bottom;

        ((left_top_x - right_bottom_x) * (left_top_y - right_bottom_y))
            .abs()
            .into()
    }

    fn perimeter(&self) -> Perimeter {
        let Point {
            x: left_top_x,
            y: left_top_y,
        } = self.left_top;
        let Point {
            x: right_bottom_x,
            y: right_bottom_y,
        } = self.right_bottom;

        (2f64 * ((left_top_x - right_bottom_x).abs() + (left_top_y - right_bottom_y).abs())).into()
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.left_top.x += x;
        self.right_bottom.x += x;

        self.left_top.y += y;
        self.right_bottom.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pair({}, {})", self.0, self.1)
    }
}

impl Pair {
    fn destroy(self) {
        println!("Удаляем {}", self);
    }
}

pub fn methods() {
    let rectangle = Rectangle {
        // Связанные функции вызываются с помощью двойных двоеточий
        left_top: Point::default(),
        right_bottom: Point::new(3f64, 4f64),
    };

    // Методы вызываются с помощью оператора "точка"
    // Обратите внимание, что первый аргумент `&self` передаётся неявно, т.е.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("{}", rectangle.perimeter());
    println!("{}", rectangle.area());

    let mut square = Rectangle {
        left_top: Point::default(),
        right_bottom: Point::new(1f64, 1f64),
    };

    // Ошибка! `rectangle` неизменяемый, но в методе требуется изменяемый
    // объект
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Попробуйте раскомментировать эту строку

    // Порядок! Изменяемые объекты могут вызывать изменяемые методы
    square.translate(1f64, 1f64);

    let pair = Pair(Box::new(0), Box::new(1));

    pair.destroy();

    // Ошибка! Предыдущий вызов `destroy` "употребил" переменную `pair`
    // pair.destroy();
    // TODO ^ Попробуйте раскомментировать эту строку
}

pub fn closures() {
    // Инкремент с помощью замыкания и функции.
    fn increment(i: i32) -> i32 {
        i + 1
    }

    // Замыкания анонимны. Тут мы связываем их с ссылками
    // Аннотация идентичны аннотации типов функции, но является опциональной
    // как и оборачивания тела в `{}`. Эти безымянные функции
    // назначены соответствующе названным переменным.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1i32;

    println!("функция: {}", increment(i));
    println!("замыкание с указанием типа: {}", closure_annotated(i));
    println!("замыкание без указания типа: {}", closure_inferred(i));

    // Замыкание не принимает аргументов, но возвращает `i32`.
    // Тип возвращаемого значения выведен автоматически.
    let one = || 1;
    println!("замыкание, возвращающее один: {}", one())
}

enum Color {
    RED,
    GREEN,
    BLUE,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::RED => write!(f, "red"),
            Color::GREEN => write!(f, "green"),
            Color::BLUE => write!(f, "blue"),
        }
    }
}

pub fn borrowing() {
    use std::mem;
    use Color::*;

    let color = GREEN;

    // Замыкание для вывода `color`, которое немедленно заимствует (`&`)
    // `color` и сохраняет замыкание в переменной `print`. `color` будет оставаться
    // заимствованным до тех пор, пока `print` не будет использован в последний раз.
    //
    // `println!` принимает аргументы только по неизменяемым ссылкам, поэтому он не накладывает
    // дополнительных ограничений.
    let print_color = || println!("color: {}", color);

    // Вызываем замыкание, использующее заимствование.
    print_color();

    // `color` может быть неизменяемо заимствован, так как замыкание
    // держит только неизменяемую ссылку на `color`.
    let _reborrow = &color;
    print_color();

    // Перемещение или перезанятие возможно после последнего использования `print`
    let _color_moved = color;
    // print_color();

    let mut count = 0i32;

    // Замыкание для увеличения `count` может принимать как `&mut count`, так и `count`,
    // но использование `&mut count` накладывает меньше ограничений, так что
    // замыкание выбирает первый способ, т.е. немедленно заимствует `count`.
    //
    // `inc` должен быть `mut`, поскольку внутри него хранится `&mut`.
    // Таким образом, вызов замыкания изменяет его, что недопустимо без `mut`.
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    // Вызываем замыкание, использующее изменяемое заимствование.
    inc();

    // Замыкание продолжает изменяемо заимствовать `count`, так как оно используется дальше.
    // Попытка перезанять приведёт к ошибке.
    // let _reborrow = &count;
    // ^ TODO: попробуйте раскомментировать эту строку.
    inc();

    // Замыкание больше не заимствует `&mut count`. Так что теперь
    // при перезаимствовании ошибок не будет.
    let _count_reborrowed = &mut count;

    // Некопируемый тип.
    let movable = Box::new(3);

    // `mem::drop` требует `T`, так что захват производится по значению.
    // Копируемый тип будет скопирован в замыкание, оставив оригинальное
    // значение без изменения. Некопируемый тип должен быть перемещён, так что
    // `movable` немедленно перемещается в замыкание.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` поглощает переменную, так что оно может быть вызвано только один раз.
    consume();
    // consume();
    // ^ TODO: Попробуйте раскомментировать эту строку.
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

pub fn functional_traits() {
    use std::mem;

    let greeting = "Привет";

    // Не копируемый тип.
    // `to_owned` преобразует заимствованные данные в собственные.
    let mut farewell = "Пока".to_owned();

    // Захват двух переменных: `greeting` по ссылке и
    // `farewell` по значению.
    let diary = || {
        // `greeting` захватывается по ссылке: требует `Fn`.
        println!("Я сказал {}.", greeting);

        // Изменяемость требует от `farewell` быть захваченным
        // по изменяемой ссылке. Сейчас требуется `FnMut`.
        farewell.push_str("!!!");
        println!("Потом я закричал {}.", farewell);
        println!("Теперь я могу поспать. zzzzz");

        // Ручной вызов удаления требуется от `farewell`
        // быть захваченным по значению. Теперь требуется `FnOnce`.
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("Удвоенное 3: {}", apply_to_3(double))
}

// `F` должен реализовать `Fn` для замыкания, которое
// ничего не принимает и не возвращает - именно то,
// что нужно для `print`.
fn apply_fn<F>(f: F)
where
    F: Fn(),
{
    f();
}

pub fn anonymous_types() {
    let x = 7;

    // Захватываем `x` в анонимный тип и реализуем
    // `Fn` для него. Сохраняем его как `print`.
    let print = || println!("{}", x);

    apply_fn(print);
}

fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("Я что-то делаю...")
}

fn increment(i: &mut i32) {
    *i += 1;
}

fn apply_digit<F: FnMut(&mut i32)>(mut f: F, digit: &mut i32) {
    f(digit)
}

pub fn income_function_parameter() {
    let closure = || println!("Я замыкание");

    call_me(closure);
    call_me(function);

    let mut count = 100;

    apply_digit(increment, &mut count);

    println!("100 + 1 = {}", count)
}

fn simple_fn() -> impl Fn() {
    let text = "Hello from Simple Fn";

    move || println!("{text}")
}

fn increment_fn() -> impl FnMut() {
    let mut count = 0i32;

    move || {
        count += 1;
        println!("Hello from Increment Fn, {count}")
    }
}

fn once_fn() -> impl FnOnce() {
    let count = Box::new(1i32);

    move || {
        std::mem::drop(count);
        println!("Hello from Once Fn")
    }
}

pub fn output_function_parameters() {
    let s_fn = simple_fn();
    let mut inc_fn = increment_fn();
    let once_fn = once_fn();

    s_fn();
    s_fn();
    s_fn();

    inc_fn();
    inc_fn();
    inc_fn();

    once_fn();
    // try uncomment line below
    // once_fn();
}

pub fn iterator_any() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut checks_count = 0;
    // `iter()` для векторов даёт `&i32`. Приводим к `i32`.
    println!(
        "2 в vec1: {}",
        vec1.iter().any(|&x| -> bool {
            checks_count += 1;
            x == 2
        })
    );

    println!("2 в vec1 checks count: {}", checks_count);

    // `into_iter()` для векторов даёт `i32`. Приведения не требуется.
    println!("2 в vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` для массивов даёт `&i32`.
    println!("2 в array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` для массивов даёт `i32`.
    println!("2 в array2: {}", array2.into_iter().any(|x| x == 2));
}

pub fn iterator_find() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` для векторов выдаёт `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` для векторов выдаёт `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` для векторов выдаёт `&i32`, а мы хотим ссылаться на один из его
    // элементов, поэтому нам нужно деструктурировать `&&i32` в `i32`
    println!("Найдём 2 в vec1: {:?}", iter.find(|&&x| x == 2));
    // `into_iter()` для векторов выдаёт `i32`, а мы хотим ссылаться на один
    // из его элементов, поэтому нам нужно деструктурировать `&i32` в `i32`
    println!("Найдём 2 в vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` для массивов выдаёт `&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // `into_iter()` для массивов выдаёт `i32`
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );
}

pub fn iterator_position() {
    let vec = vec![1, 9, 3, 3, 13, 2];

    // `iter()` для векторов выдаёт `&i32`, а `position()` не принимает ссылку, поэтому
    // мы должны деструктурировать `&i32` в `i32`
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    // `into_iter()` для векторов выдаёт `i32`, а `position()` не принимает ссылку, поэтому
    // деструктуризация не требуется
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}

fn is_odd(i: u32) -> bool {
    i % 2 == 1
}

pub fn high_order_functions() {
    println!("Найти сумму всех квадратов нечётных чисел не больше 1000");
    let upper = 1000;

    // Императивный подход
    // Объявляем переменную-накопитель
    let mut acc = 0;
    // Итерируем: 0, 1, 2, ... до бесконечности
    for n in 0.. {
        // Возводим число в квадрат
        let n_squared = n * n;

        if n_squared >= upper {
            // Останавливаем цикл, если превысили верхний лимит
            break;
        } else if is_odd(n_squared) {
            // Прибавляем число, если оно нечётное
            acc += n_squared;
        }
    }
    println!("императивный стиль: {}", acc);

    // Функциональный подход
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // Все натуральные числа возводим в квадрат
        .take_while(|&n_squared| n_squared < upper) // Берём те, что ниже верхнего предела
        .filter(|&n_squared| is_odd(n_squared)) // Выбираем нечётные
        .sum(); // Складываем
    println!("функциональный стиль: {}", sum_of_squared_odd_numbers);
}

pub fn never_type() {
    fn foo() -> ! {
        panic!("Этот вызов никогда не вернёт управление.");
    }

    fn some_fn() {
        ()
    }

    let _a: () = some_fn();
    println!("Эта функция возвращает управление и вы можете увидеть эту строку.")
}

pub fn sum_odd_numbers() {
    let up_to = 9;
    let mut acc = 0;
    for i in 0..up_to {
        // Обратите внимание, что возвращаемый тип этого выражения match должен быть u32
        // потому что такой тип в переменной "addition" .
        let addition: u32 = match i % 2 == 1 {
            // Переменная "i" типа u32, что совершенно нормально.
            true => i,
            // С другой стороны выражение "continue" не возвращает
            // u32, но это тоже нормально, потому что это тип не возвращающий управление,
            // не нарушает требования к типу выражения match.
            false => continue,
        };
        acc += addition;
    }

    println!("Сумма нечётных чисел до 9 (исключая): {}", acc);
}

// Задание: реализовать типы Map,TakeWhile,Filter которые реализуют std::iter::Iterator
