#![allow(dead_code)]

use std::mem;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new_zero() -> Self {
        Self::new(0f64, 0f64)
    }

    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn box_example() {
    let top_left = Point::new_zero();
    let bottom_right = Point::new(3f64, -4f64);

    let rectangle = Rectangle {
        top_left,
        bottom_right,
    };

    let boxed_rectangle = Box::new(Rectangle {
        top_left,
        bottom_right,
    });

    let boxed_point = Box::new(Point::new_zero());

    let box_in_a_box = Box::new(Box::new(Point::new_zero()));

    println!(
        "Точка занимает {} байт на стеке",
        mem::size_of_val(&top_left)
    );
    println!(
        "Прямоугольник занимает {} байт на стеке",
        mem::size_of_val(&rectangle)
    );

    // box size == pointer size
    println!(
        "Упакованная точка занимает {} байт на стеке",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Упакованный прямоугольник занимает {} байт на стеке",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Упакованная 'упаковка' занимает {} байт на стеке",
        mem::size_of_val(&box_in_a_box)
    );

    // Копируем данные из `boxed_point` в `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!(
        "Распакованная точка занимает {} байт на стеке",
        mem::size_of_val(&unboxed_point)
    );
}

pub fn vector_example() {
    let collected_iterator: Vec<i32> = (0i32..=10).collect();
    println!("[0..10] собраны в {:?}", collected_iterator);

    let mut xs = vec![0i32, 1, 2];
    println!("Исходный вектор {:?}", xs);

    // Вставка нового элемента в конец вектора
    println!("Добавим 4 в конец вектора");
    xs.push(4);
    println!("Вектор: {:?}", xs);

    // Ошибка! Неизменяемые вектора не могут увеличиваться
    // collected_iterator.push(0);
    // ИСПРАВЬТЕ ^ Закомментируйте эту строку

    // Метод `len` отдаёт количество элементом, сохранённых в векторе
    println!("Длина вектора: {}", xs.len());

    // Индексация выполняется при помощи квадратных скобок (индексация начинается с 0)
    println!("Второй элемент: {:?}", xs.get(1));

    // `pop` удаляет последний элемент из вектора и возвращает его
    println!("Последний элемент: {:?}", xs.pop());

    // Выход за пределы индексации вызывает панику
    // println!("Четвёртый элемент: {}", xs[3]);
    // ИСПРАВЬТЕ ^ Закомментируйте эту строку

    // По векторами легко итерироваться
    println!("Содержимое `xs`:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // Также можно итерироваться по вектору с получением индекса элемента
    // (который будет содержаться в отдельной переменной `i`)
    for (i, x) in xs.iter().enumerate() {
        println!("{}-ый элемент имеет значение {}", i, x);
    }

    // Благодаря `iter_mut`, у изменяемых векторов можно менять значения
    // во время итерирования
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Обновлённый вектор: {:?}", xs);
}

pub mod string {
    pub fn strings_example() {
        let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
        println!("Pangram: {pangram}");

        println!("Words in reverse");
        for word in pangram
            .split_whitespace()
            .rev()
        {
            println!("> {word}")
        }

        let mut chars: Vec<char> = pangram.chars().collect();
        chars.sort();
        chars.dedup();

        let mut string = String::new();

        for c in chars {
            string.push(c);
            string.push_str(", ");
        }

        let chars_to_trim = [' ', ','];
        let trimmed_str: &str = string.trim_matches(chars_to_trim);
        println!("Used characters: {trimmed_str}");

        let alice = String::from("I like dogs");
        let bob: String = alice.replace("dog", "cat");

        println!("Alice says: {}", alice);
        println!("Bob says: {}", bob);
    }

    pub fn literals() {
        // Вы можете использовать экранирование для записи байтов
        // при помощи их шестнадцатиричных значений...
        let byte_escape = "Я пишу на \x52\x75\x73\x74";
        println!("Что ты делаешь\x3F (\\x3F означает ?) {byte_escape}");

        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

        println!("Unicode символ {unicode_codepoint} (U+211D) называется {character_name}");

        let long_string = "
            Строковый литерал
            может занимать несколько строк.
            Разрыв строки и отступ ->\
            <- также можно экранировать!";
        println!("{long_string}");
    }

    pub fn raw_str() {
        let raw_str = r"Экранирование здесь не работает: \x3F \u{211D}";
        println!("{raw_str}");

        // Если вам необходимы кавычки с сырой строке, добавьте пару `#`
        let quotes = r#"И затем я сказал: "Здесь нет экранирования""#;
        println!("{quotes}");

        // Если вам необходимо добавить в вашу строку `"#`, то просто добавьте больше `#` в разделитель.
        // Здесь нет ограничений на количество `#` которое вы можете использовать.
        let longer_delimeter = r###"Строка с "# внутри неё. И даже с "##!"###;
        println!("{longer_delimeter}")
    }

    pub fn bytes() {
        let bytestring = b"it is a bytestring";
        println!("Строка байтов {bytestring:?}",);

        let escaped = b"\x52\x75\x73\x74 as bytes";
        println!("Экранированные байты {escaped:?}");

        let raw_bytestring = br"\u{u210} is not escaped here";
        println!("{raw_bytestring:?}");

        // Преобразование массива байт в `str` может завершиться ошибкой
        if let Ok(my_str) = std::str::from_utf8(raw_bytestring) {
            println!("И тоже самое в виде текста {my_str}");
        }

        let _quotes = br#"You can also use the formatting you like, \
            as with regular raw strings"#;

        // Байтовые строки не обязаны быть UTF-8
        let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82";

        // Но из-за этого они не всегда могут быть преобразованы в `str`
        #[allow(invalid_from_utf8)]
        match std::str::from_utf8(shift_jis) {
            Ok(s) => println!("Удачное преобразование: '{s}'"),
            Err(e) => println!("Неудачное преобразование: {e:?}",),
        }
    }
}

fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        Some(quotient) => println!("{dividend} / {divisor} = {quotient}"),
        None => println!("{dividend} / {divisor} вызвало ошибку!"),
    }
}

pub fn option_example() {
    try_division(4, 2);
    try_division(1, 0);

    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    let unwrapped_float = optional_float.unwrap();
    println!("{optional_float:?} распаковывается в {unwrapped_float:?}");

    let unwrapped_none = none.unwrap();
    println!("{none:?} распаковывается в {unwrapped_none:?}");
}

pub mod checked {
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarifm,
        NegativeSquareRoot,
    }

    impl std::fmt::Display for MathError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MathError::DivisionByZero => write!(f, "деление на ноль"),
                MathError::NonPositiveLogarifm => write!(f, "логарифм не положительного числа"),
                MathError::NegativeSquareRoot => {
                    write!(f, "квадратный корень от отрицательного числа")
                }
            }
        }
    }

    pub type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0f64 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0f64 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0f64 {
            Err(MathError::NonPositiveLogarifm)
        } else {
            Ok(x.ln())
        }
    }

    fn op_(x: f64, y: f64) -> MathResult {
        let ratio = div(x, y)?;

        let ln = ln(ratio)?;

        sqrt(ln)
    }

    fn op(x: f64, y: f64) {
        match op_(x, y) {
            Ok(value) => println!("{value}"),
            Err(e) => println!("{e}"),
        }
    }

    pub fn example() {
        op(100f64, 0f64);
    }
}

pub mod hash_map {
    use std::collections::HashMap;

    #[derive(PartialEq, Eq, Hash)]
    struct Account<'a> {
        username: &'a str,
        password: &'a str,
    }

    struct AccountInfo<'a> {
        age: u8,
        fio: &'a str,
        email: &'a str,
    }

    type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

    fn try_logon<'a>(accounts: &Accounts, username: &str, password: &str) {
        println!("Имя пользователя: {}", username);
        println!("Пароль: {}", password);
        println!("Попытка входа...");

        let logon = Account { username, password };

        match accounts.get(&logon) {
            Some(info) => {
                println!("Успешный вход!");
                println!("Имя: {}", info.fio);
                println!("Email: {}", info.email);
                println!("Возраст: {} лет", info.age);
            }
            None => println!("Неверный логин или пароль"),
        }
    }

    pub fn custom_hash_map_example() {
        let mut accounts = Accounts::with_capacity(1);

        let account = Account {
            username: "amidman",
            password: "123",
        };

        let account_info = AccountInfo {
            age: 20,
            fio: "Воронков Дмитрий Андреевич",
            email: "amidman@outlook.com",
        };

        accounts.insert(account, account_info);

        try_logon(&accounts, "amidwoman", "321");
        try_logon(&accounts, "amidman", "123");
    }
}

pub mod hash_set {
    use std::collections::HashSet;

    pub fn example() {
        let mut a: HashSet<i32> = vec![1i32, 2, 3]
            .into_iter()
            .collect();
        let mut b: HashSet<i32> = vec![2i32, 3, 4]
            .into_iter()
            .collect();

        let _number_inserted = a.insert(4);

        // `HashSet::insert()` вернёт `false`
        // если элемент уже содержится в наборе.
        // assert!(number_inserted, "Значение 4 уже есть в наборе A!");
        // ИСПРАВЬТЕ ^ Закомментируйте эту строку

        b.insert(5);

        // Если элементы коллекции реализуют `Debug`,
        // то и сама коллекция реализует `Debug`.
        // Обычно, элементы выводятся в формате `[elem1, elem2, ...]`
        println!("A: {:?}", a);
        println!("B: {:?}", b);

        // Выведет [1, 2, 3, 4, 5] в произвольном порядке
        println!(
            "Union: {:?}",
            a.union(&b)
                .collect::<Vec<&i32>>(),
        );

        // Выведет только [1]
        println!(
            "Difference: {:?}",
            a.difference(&b)
                .collect::<Vec<&i32>>()
        );

        // Выведет [2, 3, 4] в произвольном порядке.
        println!(
            "Intersection: {:?}",
            a.intersection(&b)
                .collect::<Vec<&i32>>()
        );

        // Выведет [1, 5]
        println!(
            "Symmetric Difference: {:?}",
            a.symmetric_difference(&b)
                .collect::<Vec<&i32>>()
        );
    }
}

pub mod rc {

    use std::rc::Rc;

    pub fn example() {
        let rc_examples = "Примеры с Rc".to_string();

        {
            println!("--- Создан rc_a ---");

            let rc_a: Rc<String> = Rc::new(rc_examples);

            // let moved_string = rc_examples;
            // Rc забирает владение ^^^ попробуйте раскоментировать эту строчку

            println!("Счётчик ссылок в rc_a: {}", Rc::strong_count(&rc_a));
            println!("Weak Счётчик ссылок в rc_a: {}", Rc::weak_count(&rc_a));

            {
                println!("--- rc_a клонировано в rc_b ---");
                let rc_b: Rc<String> = Rc::clone(&rc_a);
                println!("Счётчик ссылок в rc_b: {}", Rc::strong_count(&rc_b));
                println!("Счётчик ссылок в rc_a: {}", Rc::strong_count(&rc_a));

                // Два `Rc` равны, если равны их внутренние значения
                println!("rc_a и rc_b равны: {}", rc_a.eq(&rc_b));

                // Мы можем напрямую использовать методы внутреннего значения
                println!("Размер значения внутри rc_a: {}", rc_a.len());
                println!("Значение rc_b: {}", rc_b);

                println!("--- rc_b удаляется ---");
            }

            println!("Счётчик ссылок в rc_a: {}", Rc::strong_count(&rc_a));
            println!("Weak Счётчик ссылок в rc_a: {}", Rc::weak_count(&rc_a));

            println!("--- rc_a удаляется ---");
        }

        // let moved_string = rc_examples;
        // Rc забирает владение ^^^ попробуйте раскоментировать эту строчку
    }
}
