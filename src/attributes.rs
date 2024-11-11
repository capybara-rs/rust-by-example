#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("Вы работаете на операционной системе linux")
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("Вы работаете *не* в linux!");
}

pub fn cfg_attributes() {
    are_you_on_linux();

    let accept = if cfg!(target_os = "linux") {
        "Да, это точно linux"
    } else {
        "Да, это точно *не* linux"
    };

    println!("{accept}")
}
