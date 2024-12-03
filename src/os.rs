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

pub mod channels {}
