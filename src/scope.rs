#![allow(dead_code)]

fn destroy_box(b: Box<i32>) {
    println!("Уничтожаем упаковку, в которой хранится {}", b);

    // `c` уничтожится, а память будет освобождена
}

pub fn moving_and_ownership() {
    // Целое число выделенное в стеке
    let x = 5u32;

    // *Копируем* `x` в `y`. В данном случае нет ресурсов для перемещения
    let y = x;

    // Оба значения можно использовать независимо
    println!("x равен {}, а y равен {}", x, y);

    // `a` - указатель на целое число, выделенное в куче
    let a = Box::new(5i32);

    println!("a содержит: {}", a);

    // *Перемещаем* `a` в `b`
    let b = a;
    // Адрес указателя `a` копируется (но не данные) в `b`.
    // Оба указателя указывают на одни и те же данные в куче, но
    // `b` теперь владеет ими.

    // Ошибка! `a` больше не может получить доступ к данным, потому что
    // больше не владеет данными в куче.
    // println!("a содержит: {}", a);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Эта функция берет во владение память, выделенную в куче, которой ранее владела `b`
    destroy_box(b);

    // Поскольку в данный момент память в куче уже освобождена, это действие
    // приведёт к разыменованию освобождённой памяти, но это запрещено компилятором
    // Ошибка! Причина та же, что и в прошлый раз
    // println!("b содержит: {}", b);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку
}

pub fn mutability() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box содержит в себе {}", immutable_box);

    // Ошибка изменяемости
    //*immutable_box = 4;

    // *Переместить* упаковку, изменив её владение (и изменяемость)
    let mut mutable_box = immutable_box;

    println!("mutable_box содержит в себе {}", mutable_box);

    // Изменяем данные внутри упаковки
    *mutable_box = 4;

    println!("mutable_box now содержит в себе {}", mutable_box);
}

// Эта функция берёт во владение упаковку и уничтожает её
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Уничтожаем упаковку в которой хранится {}", boxed_i32);
}

// Эта функция заимствует i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("Это число равно: {}", borrowed_i32);
}

pub fn borrowing() {
    // Создаём упакованное i32, и i32 на стеке
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Заимствуем содержимое упаковки. При этом мы не владеем ресурсом.
    // Содержимое может быть заимствовано снова.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Получаем ссылку на данные, которые хранятся внутри упаковки
        let _ref_to_i32: &i32 = &boxed_i32;

        // Ошибка!
        // Нельзя уничтожать упаковку `boxed_i32` пока данные внутри заимствованы.
        // eat_box_i32(boxed_i32);
        // ИСПРАВЬТЕ ^ Закомментируйте эту строку

        // `_ref_to_i32` покидает область видимости и больше не является заимствованным ресурсом.
    }

    // `boxed_i32` теперь может получить владение над `eat_box` и быть уничтожено
    eat_box_i32(boxed_i32);
}

#[derive(Clone, Copy)]
struct Book {
    // `&'static str` - это ссылка на строку, расположенную в неизменяемой памяти
    author: &'static str,
    title: &'static str,
    year: i32,
}

fn borrow_book(book: &Book) {
    println!(
        "Я неизменяемо заимствовала {} - {} издания",
        book.title, book.year
    );
}

fn inc_edition(book: &mut Book) {
    book.year += 1;
    println!(
        "Я изменяемо заимствовала {} - {} издания",
        book.title, book.year
    );
}

pub fn mutability_borrowing() {
    let immutable_book = Book {
        author: "amidman",
        title: "the best book of all time",
        year: 2038,
    };

    let mut mut_book = immutable_book;

    borrow_book(&immutable_book);

    borrow_book(&mut_book);

    inc_edition(&mut mut_book);

    // inc_edition(&mut immutable_book);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn aliasing() {
    let mut point = Point {
        x: 1i32,
        y: 2i32,
        z: 0i32,
    };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Данные могут быть доступны через ссылки и владельца этих данных
    println!(
        "Точка имеет координаты: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // Ошибка! Нельзя заимствовать для изменения `point`, так как она уже
    // существует неизменяемая ссылка.
    // let mutable_borrow = &mut point;
    // TODO ^ Попробуйте раскомментировать эту строку

    // Заимствованное значение снова используется
    println!(
        "Точка имеет координаты: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // Неизменяемая ссылка больше не используется, так что можно перезаимствовать её
    // с помощью изменяемой ссылки.
    let mutable_borrow = &mut point;

    // Меняем при помощи изменяемой ссылки
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Ошибка! Нельзя неизменяемо заимствовать `point` так как она уже
    // заимствована изменяемо.
    //let y = &point.y;
    // TODO ^ Попробуйте раскомментировать эту строку

    // Ошибка! Нельзя вывести на экран, потому что `println!` берёт неизменяемую ссылку.
    // println!("Координата Z {}", point.z);
    // TODO ^ Попробуйте раскомментировать эту строку

    // Ok! Изменяемая ссылка может быть передана `println!` как неизменяемая
    println!(
        "Точка имеет координаты: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // Изменяемая ссылка больше не используется, так что можно перезаимствовать
    let new_borrowed_point = &point;
    println!(
        "Точка имеет координаты: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}

pub fn ref_pattern() {
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let c = 'Q';

    // Заимствование с `ref` по левую сторону от присваивания, эквивалетно
    // заимствованию с `&` по правую сторону.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 равно ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` также может использоваться при деструктуризации структур.
    let _copy_of_x: i32 = {
        // `ref_to_x` - ссылка на поле `x` в `point`.
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        // Возвращаем копию поля `x` из `point`.
        *ref_to_x
    };

    // Изменяемая копия `point`
    let mut mutable_point = point;

    {
        // `ref` может использоваться вместе с `mut` для получения изменяемой ссылки.
        let Point {
            x: _,
            y: ref mut ref_to_mut_y,
        } = mutable_point;

        // Изменяем поле `y` переменной `mutable_point` через изменяемую ссылку.
        *ref_to_mut_y += 1;
    }

    println!("point ({}, {})", point.x, point.y);
    println!("mutable_point ({}, {})", mutable_point.x, mutable_point.y);

    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        let (_, ref mut last) = mutable_tuple;
        // Деструктурируем `mutable_tuple` чтобы изменить значение `last`.
        *last += 100;
    }

    println!("tuple {:?}", mutable_tuple);
}

// `print_refs` получает две ссылки на `i32`, имеющие различные
// времена жизни `'a` и `'b`. Оба этих времени жизни должны существовать
// не меньше, чем функция `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x равно {} и y равно {}", x, y);
}

// Функция, не имеющая аргументов, но имеющая параметр времени жизни `'a`.
fn failed_borrow<'a>() {
    let _x = 12;

    // ОШИБКА: `_x` не живёт достаточно долго (`_x` does not live long enough)
    // let y: &'a i32 = &_x;
    // Попытка использования времени жизни `'a` для явного аннотирования
    // внутри функции приведёт к ошибке, так как время жизни у `&_x` короче, чем
    // у `y`. Короткое время жизни не может быть приведено к длинному.
}

pub fn explicit_annotation() {
    // Создадим переменные, которые далее будут заимствованы.
    let (four, nine) = (4, 9);

    // Заимствуем (`&`) обе переменные и передадим их в функцию.
    print_refs(&four, &nine);
    // Любой ввод, который заимствуется, должен жить дольше, чем заимствующий.
    // Другими словами, время жизни `four` и `nine` должно
    // быть больше, чем время жизни `print_refs`.

    failed_borrow();
    // `failed_borrow` не содержит ссылок, заставляющих `'a` быть
    // больше, чем время жизни функции, но `'a` больше.
    // Поскольку время жизни никогда не ограничено, оно, по умолчанию, равно `'static`.
}

//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// Код написанный выше является недопустимым: время жизни `'a`
// должно жить после выхода из функции.
// Здесь, `&String::from("foo")` создает ссылку на `String`
// Данные будут удалены после выхода из области видимости, и
// будет возвращена ссылка на недопустимые данные.

// Одна входная ссылка со временем жизни `'a`, которая
// будет жить как минимум до конца функции.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Использование времени жизни также возможно с изменяемыми ссылками.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Несколько элементов с различными временами жизни. В этом случае
// было бы хорошо, чтобы у обоих ссылок было одно время жизни `'a`,
// в более сложных случаях может потребоваться различное время жизни.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Возврат переданных на вход ссылок допустим.
// Однако должен быть указано правильное время жизни.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

pub fn functions() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

struct Owner(i32);

impl Owner {
    // Время жизни аннотируется как в отдельной функции.
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

pub fn methods() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}

// Тип `Borrowed`, в котором находится ссылка на `i32`.
// Ссылка на `i32` должна пережить `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Аналогично, обе ссылки расположенные здесь, должны пережить эту структуру.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// Перечисление, которое указывает на `i32` или на ссылку.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

pub fn structures() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x заимствован в {:?}", single);
    println!("x и y заимствованы в {:?}", double);
    println!("x заимствован в {:?}", reference);
    println!("y *не* заимствован в {:?}", number);
}
