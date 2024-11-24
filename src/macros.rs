#![allow(dead_code)]

// Этот простой макрос называется `say_hello`.
macro_rules! hello_world {
    // `()` указывает, что макрос не принимает аргументов.
    () => {
        // Макрос будет раскрываться с содержимым этого блока.
        println!("Hello World!");
    };
}

#[macro_export]
macro_rules! hello_rust {
    () => {
        println!("Hello Rust!")
    };
}

pub use hello_rust;

pub fn hello_world() {
    hello_world!();
}

macro_rules! create_function {
    // Этот макрос принимает аргумент идентификатора `ident` и
    // создаёт функцию с именем `$func_name`.
    // Идентификатор `ident` используют для обозначения имени переменной/функции.
    ($func_name:ident) => {
        fn $func_name() {
            // Макрос `stringify!` преобразует `ident` в строку.
            println!("Вызвана функция {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! create_public_function {
    ($func_name:ident) => {
        pub fn $func_name() {
            println!("Вызвана публичная функция {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);
create_public_function!(foobar);

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

pub fn print_macros_results() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // Напомним, что блоки тоже являются выражениями!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}

macro_rules! lex_assert {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} и {:?} это {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        );
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} или {:?} это {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        );
    };
}

pub fn assert_example() {
    lex_assert!(true; and false);
    lex_assert!(true; or false);
    lex_assert!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
}

// Макросы могут использовать знак + в списке аргументов,
// чтобы указать, какие аргументы могут повторяться хоть один раз,
// или знак *, чтобы указать, какие аргументы могут повторяться ноль или несколько раз.

// В следующем примере, шаблон, окружённый $(...),+
// будет сопоставлять одно или несколько выражений,
// разделённых запятыми. Также обратите внимание,
// что точка с запятой является необязательной в последнем случае.

macro_rules! min {
    ($x:expr) => {
        $x
    };

    ($x:expr, $($y:expr), +) => {
        std::cmp::min($x, min!($($y),+))
    };
}

pub fn recursive() {
    println!("{}", min!(1u32));
    println!("{}", min!(1u32 + 2, 2u32));
    println!("{}", min!(5u32, 2u32 * 3, 4u32));
}

macro_rules! assert_equal_len {
    ($a: expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: несоответствие размеров: {:?} {:?} {:?}",
            stringify!($func),
            $a.len(),
            stringify!($op),
            $b.len(),
        );
    };
}

macro_rules! op {
    ($func: ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = x.$method(*y);
            }
        }
    };
}

use std::ops::{Add, Mul, Sub};

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -= ,sub);

mod test {

    macro_rules! test {
        ($func: ident, $x: expr, $y: expr, $z: expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    use std::iter;

                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        };
    }

    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 1u32, 5u32, 5u32);
    test!(sub_assign, 5u32, 2u32, 3u32);
}

macro_rules! calculate {
    (eval $e: expr) => {
        let v: usize = $e;
        println!("{} = {}", stringify!($e), v);
    };

    (eval $e: expr, $(eval $es:expr), +) => {
        calculate!{
            eval $e
        }

        calculate!{
            $(eval $es), +
        }
    }
}

pub fn dsl() {
    calculate! {
        eval 1 + 1
    };

    calculate! {
        eval 2 + 4,
        eval 2 + 7
    };
}
