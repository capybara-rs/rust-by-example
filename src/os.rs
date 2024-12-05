#![allow(dead_code)]

pub mod thread {
    use std::thread;

    const NTHREADS: i32 = 10;

    pub fn example() {
        let mut children = vec![];

        for i in 0..NTHREADS {
            children.push(thread::spawn(move || println!("Hello from Thread {}", i)));
        }

        for child in children {
            _ = child.join();
        }
    }

    pub fn map_reduce_example() {
        let data = "869678977374 164718532 973270503 649591186132257556
            472396329754262496285070856234701
            860851907960690014725639383979667071
            0609417278323874766921952380795257888236
            52545930333030283758495327135744041048897
            885734297812699202164389808735488084137209
            5653216278424637452589860345374828574668 asdfas asdfasd a asdfasdf";

        println!("{}", data.len());

        let chunks = data.split_whitespace();

        let mut children = Vec::new();

        for (i, chunk) in chunks.enumerate() {
            children.push(thread::spawn(move || -> u32 {
                let result = chunk
                    .chars()
                    .filter_map(|c: char| c.to_digit(10))
                    .sum();

                println!("обработан сегмент {}, result={}", i, result);

                result
            }))
        }

        let result: u32 = children
            .into_iter()
            .map(|j: thread::JoinHandle<u32>| -> std::thread::Result<u32> { j.join() })
            .filter_map(Result::ok)
            .sum();

        println!("Финальная сумма: {}", result);
    }
}

pub mod channels {

    use std::{
        sync::mpsc::{self, Receiver, Sender},
        thread,
    };

    const NTHREADS: i32 = 3;

    pub fn example() {
        let (sd, rc): (Sender<i32>, Receiver<i32>) = mpsc::channel();

        let mut children = vec![];

        for id in 0..NTHREADS {
            let thread_sender = sd.clone();

            let child = thread::spawn(move || {
                _ = thread_sender.send(id);

                println!("поток {} завершен", id);
            });

            children.push(child);
        }

        let mut ids = Vec::with_capacity(NTHREADS as usize);

        for _ in 0..NTHREADS {
            ids.push(rc.recv());
        }

        for child in children {
            _ = child.join();
        }

        println!("{:?}", ids)
    }
}

pub mod files {
    use std::fs::File;
    use std::io::{self, BufRead, Read, Write};
    use std::path::Path;

    pub fn path_example() {
        let path = Path::new(".");

        let _display = path.display();

        let new_path = path.join("a").join("b");

        match new_path.to_str() {
            Some(s) => println!("новый путь {}", s),
            None => panic!("новый путь не является действительной UTF-8 последовательностью"),
        }
    }

    pub fn open_file() {
        let path = Path::new("hello.txt");

        let display = path.display();

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => panic!("невозможно открыть {}: {}", display, err),
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(_) => println!("содержимое {}, {}", display, s),
            Err(err) => println!("ошибка открытия файла {}", err),
        }
    }

    static LOREM_IPSUM: &str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    pub fn create_file() {
        let path = Path::new("out/lorem_ipsum.txt");
        let display = path.display();

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(err) => panic!("ошибка создания файла {}, {}", display, err),
        };

        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Ok(_) => println!("файл {} успешно создан", display),
            Err(err) => println!("ошибка создания файла {}", err),
        }
    }

    pub fn read_lines() {
        if let Ok(lines) = lines("./Cargo.toml") {
            for line in lines {
                if let Ok(line) = line {
                    println!("{}", line);
                }
            }
        }
    }

    fn lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;

        Ok(io::BufReader::new(file).lines())
    }
}

pub mod process {
    use std::{
        io::{Read, Write},
        process::{Command, Stdio},
    };

    pub fn command() {
        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .unwrap_or_else(|err| panic!("ошибка при запуске {}", err));

        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);

            println!("command executed: stdout: {}", s);
        } else {
            let s = String::from_utf8_lossy(&output.stderr);

            println!("command executed, stderr: {}", s);
        }
    }

    static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

    pub fn pipes() {
        let process = Command::new("wc")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|err| panic!("spawn wc command error {}", err));

        process
            .stdin
            .unwrap()
            .write_all(PANGRAM.as_bytes())
            .unwrap_or_else(|err| panic!("failed write pangram to stdin, {}", err));

        let mut s = String::with_capacity(PANGRAM.len());
        process
            .stdout
            .unwrap()
            .read_to_string(&mut s)
            .unwrap_or_else(|err| panic!("read data from stdout, {}", err));

        println!("{}", s)
    }

    pub fn wait() {
        Command::new("sleep")
            .arg("5")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        println!("достигнут конец ожидания");
    }
}

pub mod fs {
    use std::fs::{self, File};
    use std::io::{self, Read, Write};
    use std::os::unix::fs as unix_fs;
    use std::path::Path;

    fn cat<P: AsRef<Path>>(path: P) -> io::Result<String> {
        let mut f = File::open(path)?;

        let mut s = String::new();

        f.read_to_string(&mut s)?;

        Ok(s)
    }

    fn echo<P: AsRef<Path>>(s: &str, path: P) -> io::Result<()> {
        let mut f = File::create(path)?;

        f.write_all(s.as_bytes())
    }

    fn touch<P: AsRef<Path>>(path: P) -> io::Result<()> {
        File::create(path)?;

        Ok(())
    }

    pub fn example() {
        _ = fs::remove_dir_all("a");

        let command = "`mkdir a`";

        fs::create_dir("a").unwrap_or_else(|err| panic!("{command}, {err}"));

        let command = "`echo hello > a/b.txt`";
        println!("{command}");

        echo("hello", "a/b.txt").unwrap_or_else(|err| panic!("{command}, {err}",));

        let command = "`mkdir -p a/c/d`";

        println!("{command}");

        fs::create_dir_all("a/c/d").unwrap_or_else(|err| panic!("{command}, {err}"));

        let command = "`touch a/c/e.txt`";
        touch("a/c/e.txt").unwrap_or_else(|err| panic!("{command}, {err}"));

        let command = "`ln -s ../b.txt a/c/b.txt`";
        println!("{command}");

        if cfg!(target_family = "unix") {
            unix_fs::symlink("../b.txt", "a/c/b.txt")
                .unwrap_or_else(|err| panic!("{command}, {err}"));

            println!("symlink created");
        }

        let command = "`cat a/c/b.txt`";
        println!("{command}");

        cat("a/c/b.txt").unwrap_or_else(|err| panic!("{command}, {err}"));

        let command = "`ls a`";

        println!("{command}");

        let paths = fs::read_dir("a").unwrap_or_else(|err| panic!("{command}, {err}"));
        for path in paths {
            println!("> {:?}", path.unwrap().path());
        }

        let command = "`rm a/c/e.txt`";

        println!("{command}");

        fs::remove_file("a/c/e.txt").unwrap_or_else(|err| panic!("{command}, {err}"));

        let command = "`rmdir a/c/d`";

        fs::remove_dir("a/c/d").unwrap_or_else(|err| panic!("{command}, {err}"));
    }
}

pub mod args {
    use core::fmt;
    use std::{
        env::{self},
        process::exit,
    };

    pub fn example() {
        let args: Vec<String> = env::args().collect();

        println!("Путь до программы, {}", args[0]);

        println!("У меня {} аргумента: {:?}", args.len() - 1, &args[1..]);
    }

    fn help() {
        println!(
            "usage:\
            first arg: digit\
            second arg: operator (+ or -)\
            third arg: digit\
            examples: \
            \t1 + 1\
            \t2 - 2"
        )
    }

    struct OperatorError {
        raw: String,
    }

    impl fmt::Display for OperatorError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "invalid operator: {}", self.raw,)
        }
    }

    enum Operator {
        Plus,
        Minus,
    }

    impl fmt::Display for Operator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Operator::Plus => write!(f, "+"),
                Operator::Minus => write!(f, "-"),
            }
        }
    }

    impl TryFrom<&str> for Operator {
        type Error = OperatorError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "+" => Ok(Self::Plus),
                "-" => Ok(Self::Minus),
                _ => Err(OperatorError {
                    raw: value.to_owned(),
                }),
            }
        }
    }

    impl Operator {
        fn operate(&self, first: i32, second: i32) -> i32 {
            match self {
                Operator::Plus => first + second,
                Operator::Minus => first - second,
            }
        }
    }

    pub fn parse_example() {
        let args: Vec<String> = env::args().collect();

        match args.len() {
            1 => {
                println!("Вызов выполнен без дополнительных аргументов")
            }
            2 => match args[1].parse::<i32>() {
                Ok(digit) => println!("{} = {}", digit, digit),
                Err(_) => println!("первый аргумент не является числом"),
            },
            3 => {
                let first_digit = match args[1].parse::<i32>() {
                    Ok(digit) => digit,
                    Err(_) => {
                        println!("первый аргумент не является числом");

                        exit(2);
                    }
                };

                let operator: Operator = match args[2].as_str().try_into() {
                    Ok(operator) => operator,
                    Err(err) => {
                        eprintln!("{err}");

                        exit(2);
                    }
                };

                let mut second_digit = String::new();

                std::io::stdin()
                    .read_line(&mut second_digit)
                    .unwrap_or_else(|err| {
                        eprintln!("{err}");

                        exit(2);
                    });

                let second_digit = second_digit.trim_end_matches("\n");

                let second_digit = match second_digit.parse::<i32>() {
                    Ok(digit) => digit,
                    Err(_) => {
                        eprintln!("второй аргумент не является числом");

                        exit(2);
                    }
                };

                println!(
                    "{} {} {} = {}",
                    first_digit,
                    operator,
                    second_digit,
                    operator.operate(first_digit, second_digit)
                )
            }
            4 => {
                let first_digit = match args[1].parse::<i32>() {
                    Ok(digit) => digit,
                    Err(_) => {
                        eprintln!("первый аргумент не является числом");

                        exit(2);
                    }
                };

                let operator: Operator = match args[2].as_str().try_into() {
                    Ok(operator) => operator,
                    Err(err) => {
                        eprintln!("{err}");

                        exit(2);
                    }
                };

                let second_digit = match args[3].parse::<i32>() {
                    Ok(digit) => digit,
                    Err(_) => {
                        println!("первый аргумент не является числом");

                        exit(2);
                    }
                };

                println!(
                    "{} {} {} = {}",
                    first_digit,
                    operator,
                    second_digit,
                    operator.operate(first_digit, second_digit)
                )
            }
            _ => {
                eprintln!("invalid argument count");
                exit(2);
            }
        }
    }
}

pub mod ffi {
    use core::fmt;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Complex {
        re: f32,
        im: f32,
    }

    impl fmt::Debug for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.im < 0. {
                write!(f, "{}-{}i", self.re, -self.im)
            } else {
                write!(f, "{}+{}i", self.re, self.im)
            }
        }
    }

    #[link(name = "m")]
    extern "C" {
        fn csqrtf(z: Complex) -> Complex;
        fn ccosf(z: Complex) -> Complex;
    }

    fn cos(x: Complex) -> Complex {
        unsafe { ccosf(x) }
    }

    pub fn example() {
        let z = Complex {
            re: -1f32,
            im: 0f32,
        };

        let z_sqrt = unsafe { csqrtf(z) };

        println!("квадратный корень от {:?} равен {:?}", z, z_sqrt);

        // вызов безопасного API в котором находится unsafe операция
        println!("cos({:?}) = {:?}", z, cos(z));
    }
}
