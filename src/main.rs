mod attributes;
mod expressions;
mod flow_control;
mod formatting;
mod functions;
mod generics;
mod inner;
mod modules;
mod primitives;
mod transforming;
mod user_types;

fn main() {}

pub fn hierarchy() {
    inner::hello::hello();
    // inner::inner::hello();
}
