#![allow(dead_code)]

use core::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u16,
}

// unit-структура
struct Unit;

// Кортежная структура
struct Pair(i32, f32);

// Структура с двумя полями
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn square(self, len: f32) -> Rectangle {
        let bottom_right = Point {
            x: self.x + len,
            y: self.y - len,
        };

        Rectangle {
            top_left: self,
            bottom_right,
        }
    }
}

// Структуры могут быть использованы в качестве полей другой структуры

#[derive(PartialEq, Debug)]
struct Rectangle {
    // Прямоугольник может быть определён по расположению в пространстве
    // его верхнего левого и нижнего правого углов
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let width = self.bottom_right.x - self.top_left.x;
        let height = self.top_left.y - self.bottom_right.y;

        width * height
    }
}

#[allow(dead_code, unused)]
pub fn structures() {
    // Создадим структуру при помощи сокращённой инициализации полей
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Распечатаем отладочную информацию о структуре
    println!("{:?}", peter);

    // Инициализируем `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Получаем доступ к полям структуры
    println!("координаты точки: ({}, {})", point.x, point.y);

    // Создадим новую точку, используя синтаксис обновления структуры и нашу
    // существующую точку
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` будет тем же самым, что и `point.y`, так как мы взяли
    // это поле из `point`
    println!("вторая точка: ({}, {})", bottom_right.x, bottom_right.y);

    // Деструктурируем структуру при помощи `let`
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        // создание структуры также является выражением
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    // Создадим unit-структуру
    let unit = Unit;

    // Создадим кортежную структуру
    let pair = Pair(1, 0.1);

    // Доступ к полям кортежной структуры
    println!("pair содержит {:?} и {:?}", pair.0, pair.1);

    // Деструктурируем кортежную структуру
    let Pair(integer, decimal) = pair;

    println!("pair содержит {:?} и {:?}", integer, decimal);

    let top_left = Point { x: 0f32, y: 5f32 };

    let rectangle = Rectangle {
        top_left,
        bottom_right: Point { x: 5f32, y: 0f32 },
    };

    println!(
        "square with len 5 of\n{:#?}\nequal\n{:#?} is {}",
        top_left,
        rectangle,
        top_left.square(5f32) == rectangle
    );
    println!(
        "rectarea of\n{:#?}\nis {}",
        rectangle,
        rectangle.rect_area()
    );
}

// Создаём `enum` для классификации web-событий. Обратите внимание,
// как имена и информация о типе определяют вариант:
// `PageLoad != PageUnload` и `KeyPress(char) != Paste(String)`.
// Все они разные и независимые.
enum WebEvent {
    // `enum` может быть как `unit-подобным`,
    PageLoad,
    PageUnload,
    // так и кортежной структурой,
    KeyPress(char),
    Paste(String),
    // или С-подобной структурой.
    Click { x: i64, y: i64 },
}

impl WebEvent {
    fn inspect(&self) {
        match self {
            Self::PageLoad => println!("страница загружена"),
            Self::PageUnload => println!("страница выгружена"),
            // Извлечём `c` из `enum`.
            Self::KeyPress(c) => println!("нажата '{}'.", c),
            Self::Paste(s) => println!("нажата \"{}\".", s),
            // Разберём `Click` на `x` и `y`.
            Self::Click { x, y } => {
                println!("кликнуто на x={}, y={}.", x, y);
            }
        }
    }
}

pub fn enums() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` создаст `String` из строкового среза.
    let pasted = WebEvent::Paste("мой текст".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    pressed.inspect();
    pasted.inspect();
    click.inspect();
    load.inspect();
    unload.inspect();
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

pub fn enums_use_declaration() {
    use Status::*;
    use Work::*;

    let status = Poor;
    let work = Soldier;

    match status {
        Rich => println!("Деньги не самое важное в жизни"),
        Poor => println!("А в пятерочке майонез на 50 рублей дешевле"),
    }

    match work {
        Civilian => println!("На гражданке прохлаждается"),
        Soldier => println!("Здравствуй небо в облаках, здравствуй юность в сапогах"),
    }
}

// enum с неявным дискриминатором (начинается с 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum с явным дискриминатором
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn c_like_enums() {
    // `enums` может быть преобразован в целочисленное значение.
    println!("нулевой элемент {}", Number::Zero as i32);
    println!("первый элемент {}", Number::One as i32);

    println!("красный цвет #{:06x}", Color::Red as i32);
    println!("голубой цвет #{:06x}", Color::Blue as i32);
}

enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T>
where
    T: fmt::Display,
{
    fn new() -> Self {
        Self::default()
    }

    fn prepend(self, item: T) -> List<T> {
        Self::Cons(item, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, prev) => 1 + prev.len(),
        }
    }

    fn stringify(&self) -> String {
        match self {
            List::Nil => String::from("Nil"),
            List::Cons(item, prev) => format!("{}, {}", prev.stringify(), item),
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::Nil
    }
}

pub fn linked_list() {
    let mut list = List::default();

    list = list.prepend(1);
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(5);

    println!("Размер связного списка: {}", list.len());
    println!("Элементы связного списка: {}", list.stringify())
}

// Константы объявлены в глобальной области видимости.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Получаем доступ к константе внутри функции
    n > THRESHOLD
}

pub fn constants() {
    let n = 16;

    // Получаем доступ к константе внутри функции main
    println!("Это язык {}", LANGUAGE);
    println!("Установим предел, равный {}", THRESHOLD);
    println!(
        "Число {} {} предела",
        n,
        if is_big(n) {
            "больше"
        } else {
            "меньше"
        }
    );

    // Ошибка! `const` нельзя изменить.
    // THRESHOLD = 5;
    // ИСПРАВЬТЕ ^ Закомментируйте эту строчку
}
