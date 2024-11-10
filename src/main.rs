mod expressions;
mod flow_control;
mod formatting;
mod functions;
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
