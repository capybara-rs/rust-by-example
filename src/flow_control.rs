#![allow(dead_code, unreachable_code, unused_labels)]

use std::fmt;

pub fn if_else_flow_control(n: i32) {
    if n < 0 {
        print!("{} — отрицательное", n);
    } else if n > 0 {
        print!("{} — положительное", n);
    } else {
        print!("{} — нуль", n);
    }

    let big_n = if n.abs() < 10 {
        println!(", малое по модулю число, умножим его в десять раз");

        // Это выражение вернёт `i32`.
        10 * n
    } else {
        println!(", большое по модулю число, уменьшим его вдвое");

        // И это выражение вернёт `i32`.
        n / 2
        // ЗАДАНИЕ ^ Попробуйте отбросить значение, добавив точку с запятой.
    };
    //   ^ Не забудьте добавить тут точку с запятой! Все операторы `let` требуют её..

    println!("{} -> {}", n, big_n);
}

pub fn loop_flow_control() {
    let mut count = 0u32;

    println!("Будем считать до бесконечности!");

    loop {
        count += 1;

        match count {
            1 => {
                println!("Раз");
            }
            2 => {
                println!("Два");
            }
            3 => {
                println!("Три")
            }
            4 => {
                println!("Четыре")
            }
            5 => {
                println!("Пять, Заебался я считать");

                break;
            }
            _ => {}
        }
    }
}

pub fn named_loop_flow_control() {
    'outer: loop {
        println!("Кощей цыпленку, Прием, я внутри");

        'inner: loop {
            println!("Уже Глубже");

            break 'outer;
        }

        println!("Эта точка не будет достигнута");
    }

    println!("Вышли");
}

pub fn let_loop_flow_control() {
    let mut counter = 2;

    let result: i32 = loop {
        counter += 1;

        if counter % 17 == 0 {
            break counter * 11;
        }
    };

    println!("result: {}", result)
}

pub fn while_flow_control() {
    let mut n = 1;

    while n < 67 {
        print_fizz_buzz(n);

        n += 1
    }
}

pub fn for_range() {
    for n in 1..101 {
        print_fizz_buzz(n);
    }
}

pub fn for_range_equals() {
    for n in 1..=100 {
        print_fizz_buzz(n);
    }
}

fn print_fizz_buzz(n: i32) {
    if n % 15 == 0 {
        println!("fizzbuzz");
    } else if n % 3 == 0 {
        println!("fizz");
    } else if n % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

pub fn for_iter() {
    let names = vec!["Маша", "Саша", "Боб"];

    for name in names.iter() {
        match name {
            &"Боб" => println!("Импостер дядя Боб"),
            name => println!("Привет, {}", name),
        }
    }

    println!("Имена {:?}", names)
}

pub fn for_into_iter() {
    let names = vec!["Маша", "Саша", "Боб"];

    for name in names.into_iter() {
        match name {
            "Боб" => println!("Импостер дядя Боб"),
            name => println!("Привет, {}", name),
        }
    }

    // Так как мы уже позаимствовали вектор names в строчке
    // for name in names.into_iter() {
    // то далее мы не имеем к нему доступа
    // println!("Имена {:?}", names);
}

pub fn for_iter_mut() {
    let mut names = vec!["Маша", "Саша", "Боб"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Боб" => "Привет, Импостер дядя Боб",
            _ => "Привет",
        };
    }

    println!("{:?}", names);
}

pub fn match_flow_control(number: u32) {
    println!("Расскажите мне о {}", number);

    match number {
        1 => println!("Один!"),
        2 | 3 | 4 | 5 | 7 | 11 => println!("Это простое число"),
        13..=19 => println!("От 13 до 19 включительно"),
        _ => println!("Ничего особенного"),
    }

    let boolean = true;
    // Match ещё и выражение
    let binary = match boolean {
        // Ветви оператора match должны перечислять все возможные значения
        false => 0,
        true => 1,
        // TODO ^ Попробуйте закомментировать одну из этих ветвей
    };

    println!("{} -> {}", boolean, binary);
}

pub fn match_tuple_flow_control() {
    let triple = (123, -2, 4);
    // TODO ^ Поэкспериментируйте со значениями `triple`

    println!("Расскажите мне о {:?}", triple);
    // Для деструктурирования можно использовать match
    match triple {
        // Деструктурируем второй и третий элементы
        (0, y, z) => println!("Первый равен `0`, `y` равен {:?}, а `z` равен {:?}", y, z),
        (1, ..) => println!("Первый равен `1`, а остальное не важно"),
        (.., 2) => println!("Последний равен `2`, а остальное не важно"),
        (3, .., 4) => println!("Первый равен `3`, последний равен `4`, а остальное не важно"),
        // `..` можно использовать, чтобы игнорировать оставшуюся часть кортежа
        _ => println!("Не важно чему они равны"),
        // `_` означает, значение не будет присвоено переменной
    }
}

enum Color {
    Red,
    Green,
    Blue,

    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Красный"),
            Color::Green => write!(f, "Зелёный"),
            Color::Blue => write!(f, "Синий"),
            Color::RGB(r, g, b) => write!(f, "Красный: {}, Зеленый: {}, Синий: {}", r, g, b),
            Color::HSV(h, s, v) => {
                write!(f, "Тон: {}, Насыщенность: {}, Значение: {}", h, s, v)
            }
            Color::HSL(h, s, l) => write!(f, "Тон: {}, Насыщенность: {}, Яркость: {}", h, s, l),
            Color::CMY(c, m, y) => write!(f, "Голубой: {}, Пурпурный: {}, Жёлтый: {}", c, m, y,),
            Color::CMYK(c, m, y, k) => {
                write!(
                    f,
                    "Голубой: {}, Пурпурный: {}, Жёлтый: {}, Key (Черный) {}",
                    c, m, y, k
                )
            }
        }
    }
}

pub fn match_enum() {
    println!("{}", Color::RGB(1, 2, 3))
}

pub fn match_ref() {
    let ref number_reference = 5;
    let _number_reference = &5;

    match number_reference {
        &number => println!("Получаем значение через деструктуризацию: {number}",),
    }

    match *number_reference {
        number => println!("Получаем значение через разыменование: {number}",),
    }

    let number = 5;
    let mut mut_number = 5;

    match number {
        ref number => print_number_reference(number),
    }

    match mut_number {
        ref mut mut_number => {
            inc_number_reference(mut_number);
            println!("Увеличили число в 100 раз: {mut_number}")
        }
    }
}

fn print_number_reference(num: &i32) {
    println!("Вывожу ссылку на число: {num}",)
}

fn inc_number_reference(num: &mut i32) {
    *num *= 100
}

pub fn structure_destructurization() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 3), y: 5 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {a}, b = {b}, y = {y}");

    let Foo { x, y } = foo;
    println!("x = {:?}, y = {:?}", x, y);

    let Foo { x, .. } = foo;
    println!("x = {:?}", x)

    // следующий код выдаст ошибку: в шаблоне нет упоминания поля `x`
    // let Foo { y } = foo;
}

pub fn template_limiters() {
    struct Pair(i32, i32);

    let pair = Pair(10, 1);

    match pair {
        Pair(x, y) if x == y => println!("Оба аргумента пары равны"),
        Pair(x, y) if x > y => println!("Первый аргумент больше второго"),
        Pair(x, y) if x < y => println!("Первый аргумент меньше второго"),
        _ => println!("Ничего особенного"),
    }
}

fn age() -> u32 {
    return 11;
}

pub fn binding_flow_control() {
    println!("Скажи мне свой возраст");

    match age() {
        0 => println!("Младенец"),
        n @ 1..=12 => println!("Ребенок возраста {n}"),
        n @ 13..18 => println!("Подросток возраста {n}"),
        n => println!("Живу уже {n} года"),
    }
}

fn some_number() -> Option<u32> {
    Some(42)
}

pub fn enum_binding_flow_control() {
    match some_number() {
        Some(n @ 42) => println!("Ответ на вопрос жизни, смерти и всего такого: {n}"),
        Some(n) => println!("Число равно {n}"),
        None => println!("None"),
    }
}

pub fn if_let_enum_flow_control() {
    // Все переменные типа `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // Конструкция `if let` читает, как: "Если `let` деструктуризирует `number` в
    // `Some(i)`, выполнить блок (`{}`).
    if let Some(i) = number {
        println!("Соответствует {:?}!", i);
    }

    // Если нужно указать, что делать, в случае ошибки, можно добавить else:
    if let Some(i) = letter {
        println!("Соответствует {:?}!", i);
    } else {
        // Ошибка деструктуризации. Переходим к обработке ошибки.
        println!("Не соответствует числу. Давайте попробуем строку!");
    }

    // Добавляем ещё одну ситуацию несоответствия образцу.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Соответствует {:?}!", i);
    // Оцените условие `else if`, чтобы увидеть,
    // должна ли быть альтернативная ветка отказа:
    } else if i_like_letters {
        println!("Не соответствует числу. Давайте попробуем строку!");
    } else {
        // Рассматриваем ложное условие. Эта ветвь по умолчанию:
        println!("Мне не нравится сравнивать строки. Давайте возьмём смайлик :)!");
    }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

pub fn enum_flow_control_task() {
    let a = Foo::Bar;

    if let Foo::Bar = a {
        println!("Foo::Bar")
    }
}

/*
// Создадим переменную `optional` с типом `Option<i32>`
let mut optional = Some(0);

// Неоднократно повторим наш тест.
loop {
    match optional {
        // Если `optional` деструктурируется, выполним следующий блок.
        Some(i) => {
            if i > 9 {
                println!("Больше 9, уходим отсюда!");
                optional = None;
            } else {
                println!("`i` равен `{:?}`. Попробуем еще раз.", i);
                optional = Some(i + 1);
            }
            // ^ Требует 3 уровня вложенности!
        },
        // Выходим из цикла в случае ошибки деструктуризации:
        _ => { break; }
        // ^ Зачем это нужно? Должен быть способ сделать это лучше!
    }
}
*/

pub fn while_let_flow_control() {
    let mut optional = Some(100);

    while let Some(ref mut a) = optional {
        if *a > 10 {
            println!("Число слишком большое, {a}");

            *a -= 1;

            continue;
        }

        if *a == 10 {
            println!("Число равно 10");

            break;
        }
    }
}
