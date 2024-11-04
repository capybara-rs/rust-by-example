use std::fmt;

#[allow(dead_code)]
pub fn formatting() {
    // `{}` автоматически будет заменено на
    // аргументы. Они будут преобразованы в строку.
    println!("{} дней", 31);

    // Без суффиксов, 31 является i32. Можно изменить тип 31,
    // используя суффикс.

    // Существует множество способов работы с форматированным выводом. Можно указать
    // позицию для каждого аргумента.
    println!("{0}, это {1}. {1}, это {0}", "Алиса", "Боб");

    // Так же можно именовать аргументы.
    println!(
        "{subject} {verb} {object}",
        object = "ленивую собаку",
        subject = "быстрая коричневая лиса",
        verb = "прыгает через"
    );

    println!(
        "{} из {:b} людей знают, что такое двоичный код, а остальные нет.",
        1, 2
    );

    // Можно выравнивать текст, сдвигая его на указанную ширину.
    // Данный макрос отобразит в консоли
    // "     1". 5 пробелов и "1".
    println!("{number:>width$}", number = 1, width = 6);

    // Можно добавить к цифрам пару нулей. Данный макрос выведет "000001".
    println!("{number:0>width$}", number = 1, width = 6);

    // Компилятор обязательно проверит, что в макрос передано правильное количество
    // аргументов.
    println!("Меня зовут {0}, {1} {0}", "Бонд", "Джеймс");
    // ИСПРАВЬТЕ ^ Добавьте недостающий аргумент: "Джеймс"

    // Создаём структуру, которая хранит в себе `i32`. Назовём её `Structure`.
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // Однако, пользовательские типы данных, например, как эта структура
    // требуют более сложной обработки для вывода. Данный код не будет работать.
    println!(
        "Эта структура `{}` не хочет выводится на экран...",
        Structure(3)
    );
    // ИСПРАВЬТЕ ^ Закомментируйте эту строку.

    let name = "Николя";

    println!("Привет, меня зовут {name}")
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Structure(i32);

#[allow(dead_code)]
#[derive(Debug)]
struct Deep(Structure);

#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u16,
}

#[allow(dead_code)]
pub fn debug_formatting() {
    let structure = Structure(42);
    let deep = Deep(structure);

    println!(
        "debug print,\nstructure: {:?}\ndeep: {:?}",
        &structure, &deep
    );
    println!(
        "extended debug print,\nstructure: {:#?}\n deep: {:#?}",
        &structure, &deep
    );

    let person = Person {
        name: "Дмитрий",
        age: 19,
    };

    println!("debug print for Person struct,\n person: {:?}", &person);
    println!(
        "extended debug print for Person struct,\n person: {:#?}",
        &person,
    )
}

// Структура, которая хранит в себе два числа.
// Вывод типажа `Debug` добавлен для сравнения с `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Реализуем `Display` для `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Используем `self.номер`, чтобы получить доступ к каждому полю структуры.
        write!(f, "min: {}, max: {}", self.0, self.1)
    }
}

// Объявим структуру с именованными полями, для сравнения
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// По аналогии, реализуем `Display` для Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Обращаться к полям структуры Point2D будет по имени
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}{:b}", self.x.to_bits(), self.y.to_bits())
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[allow(dead_code)]
pub fn display_formatting() {
    let minmax = MinMax(0, 14);

    println!("Сравниваем структуры:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "Большой диапазон - {big} и маленький диапазон {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Сравниваем точки:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Сравниваем комплексные числа:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    // Ошибка. Типажи `Debug` и `Display` были реализованы, но `{:b}`
    // необходима реализация `fmt::Binary`. Следующий код не сработает.
    println!("Как выглядит Point2D в виде двоичного кода: {:b}", point);
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (index, elem) in vec.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}: {}", index, elem)?;
        }

        write!(f, "]")
    }
}

#[allow(dead_code)]
pub fn list_formatting() {
    let list = List(vec![1, 2, 3, 4]);

    println!("List: {}", list)
}

#[allow(dead_code)]
struct City {
    name: &'static str,
    // Широта
    lat: f32,
    // Долгота
    lon: f32,
}

impl fmt::Display for City {
    // `f` — это буфер, данный метод должен записать в него форматированную строку
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` похож на `format!`, только он запишет форматированную строку
        // в буфер (первый аргумент функции)
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:X}{:X}{:X}",
            self.red, self.green, self.blue, self.red, self.green, self.blue,
        )
    }
}

#[allow(dead_code)]
pub fn extended_formatting() {
    let cities = [
        City {
            name: "Дублин",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Осло",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Ванкувер",
            lat: 49.25,
            lon: -123.1,
        },
    ];

    for city in cities.iter() {
        println!("{}", city);
    }

    let colors = [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ];

    for color in colors.iter() {
        println!("{}", color)
    }
}
