#![allow(dead_code)]

// Модуль по имени `my_mod`
mod my_mod {
    // Все элементы модуля по умолчанию являются приватными.
    fn private_function() {
        println!("вызвана `my_mod::private_function()`")
    }

    // Используем модификатор `pub`, чтобы сделать элемент публичным.
    pub fn function() {
        println!("вызвана `my_mod::function()`")
    }

    // Приватные элементы модуля доступны другим элементам данного модуля.
    pub fn indirect_access() {
        print!("вызвана `my_mod::indirect_access()`, которая\n>");
        private_function();
    }

    // Модули так же могут быть вложенными
    pub mod nested {
        pub fn function() {
            println!("вызвана `my_mod::nested::function()`")
        }

        fn private_function() {
            println!("вызвана `my_mod::nested::private_function()`")
        }

        pub(in crate::modules::my_mod) fn public_function_in_my_mod() {
            print!("вызвана `my_mod::nested::public_function_in_my_mod()`, которая\n > ");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested() {
            println!("вызвана `my_mod::nested::public_function_in_nested");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("вызвана my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("вызвана `my_mod::call_public_funcion_in_my_mod()`, которая\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`")
    }

    // Вложенные модули подчиняются тем же правилам видимости
    mod private_nested {
        pub fn function() {
            println!("вызвана `my_mod::private_nested::function()`")
        }
    }
}

fn function() {
    println!("вызвана `function()`");
}

pub fn visibility() {
    // Модули позволяют устранить противоречия между элементами,
    // которые имеют одинаковые названия.
    function();
    my_mod::function();

    // Публичные элементы, включая те, что находятся во вложенном модуле,
    // доступны извне родительского модуля
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) элементы можно вызвать от везде в этом же пакете
    my_mod::public_function_in_crate();

    // pub(in path) элементы могут вызываться только для указанного модуля
    // Ошибка! функция `public_function_in_my_mod` приватная
    // my_mod::nested::public_function_in_my_mod();
    // TODO ^ Попробуйте раскомментировать эту строку

    // Приватные элементы модуля не доступны напрямую,
    // даже если вложенный модуль является публичным:

    // Ошибка! функция `private_function` приватная
    // my_mod::private_function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Ошибка! функция `private_function` приватная
    // my_mod::nested::private_function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Ошибка! Модуль `private_nested` является приватным
    //my_mod::private_nested::function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку
}

pub mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

struct Digit(i32);

impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Число {}", self.0)
    }
}

pub fn structures_visibility() {
    use my::*;

    // Публичные структуры с публичными полеми могут быть созданы как обычно,
    let open_box = OpenBox { contents: Digit(3) };

    // а их поля доступны всем.
    println!("Открытая упаковка хранит: {}", open_box.contents);

    // Публичные структуры с приватными полями не могут быть созданы с использованием имён полей
    // Ошибка! `ClosedBox` имеет приватные поля
    // let closed_box = my::ClosedBox { contents: "классифицированную информацию" };
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Однако, структуры с приватными полями могут быть созданы с помощью
    // публичного конструктора
    let _closed_box = my::ClosedBox::new(Digit(5));

    // нельзя получить доступ к приватным полям публичных структур.
    // Ошибка! Поле `contents` приватное
    // println!("Закрытая упаковка хранит: {}", _closed_box.contents);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку
}

mod use_declaration {
    pub mod nested {
        pub fn my_first_function() {
            println!("вызвана `use_declaration::nested::my_first_function()`")
        }
        pub fn my_second_function() {
            println!("вызвана `use_declaration::nested::my_second_function()`")
        }
        pub fn function() {
            println!("вызвана `use_declaration::nested::function()")
        }

        pub trait Hello {
            fn hello(&self);
        }
    }
}

pub fn use_declaration() {
    use use_declaration::nested::{
        function as my_third_function, my_first_function, my_second_function, Hello,
    };

    my_first_function();
    my_second_function();
    // вызывает use_declaration::nested::function
    my_third_function();

    {
        use use_declaration::nested::function;

        function();
    }

    struct Human;

    impl Hello for Human {
        fn hello(&self) {
            println!("Привет, Человек!")
        }
    }

    let human = Human {};

    human.hello()
}

mod cool {
    pub fn function() {
        println!("вызвана `cool::function()`")
    }
}

mod super_self {
    fn function() {
        println!("вызвана my::function()`")
    }

    mod cool {
        pub fn function() {
            println!("вызвана `my::cool::function()`")
        }
    }

    pub fn indirect_call() {
        // Давайте вызовем  все функции под названием `function` в этой области видимости!
        print!("вызвана `my::indirect_call()`, с помощью которой\n> ");

        // Ключевое слово `self` ссылается на область видимости текущего модуля.
        // В нашем случае - модуль `my`.
        // Вызов `self::function()`, так же как и вызов `function()` дают одинаковый результат,
        // т.к они ссылаются на одну и ту же функцию.
        self::function();
        function();

        // Мы так же можем использовать ключевое слово `self`,
        // чтобы получить доступ к другим модулям внутри модуля `my`:
        self::cool::function();

        // Ключевое слово `super` ссылается на родительскую область видимости (вне модуля `my`).
        super::function();

        // Этим действием мы свяжем `cool::function` в области видимости *контейнера*.
        // В данном случае область видимости контейнера является самой дальней областью видимости.
        {
            use cool::function as root_function;
            root_function();
        }
    }
}

pub fn super_and_self() {
    super_self::indirect_call();
}
