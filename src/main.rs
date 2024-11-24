mod attributes;
mod expressions;
mod flow_control;
mod formatting;
mod functions;
mod generics;
mod inner;
mod macros;
mod modules;
mod primitives;
mod scope;
mod traits;
mod transforming;
mod user_types;

fn main() {
    macros::hello_rust!();
}

pub fn hierarchy() {
    inner::hello::hello();
    // inner::inner::hello();
}
