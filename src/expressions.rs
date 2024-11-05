#![allow(dead_code)]

pub fn expressions() {
    let x = 5i32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = { 2 * x };

    println!("x равен {}", x);
    println!("y равен {}", y);
    println!("z равен {}", z);
}
