#![allow(dead_code)]

struct Human {
    age: u16,
}

pub fn moved_poiter() {
    let human = Human { age: 100 };

    println!("{:p}", &human);

    let moved_human = human;

    println!("{:p}", &moved_human);

    let new_moved_human = moved_human;

    println!("{:p}", &new_moved_human);
}
