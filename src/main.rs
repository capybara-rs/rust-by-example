mod attributes;
mod collections;
mod error;
mod experiment;
mod expressions;
mod flow_control;
mod formatting;
mod functions;
mod generics;
mod inner;
mod macros;
mod modules;
mod os;
mod primitives;
mod scope;
mod std;
mod traits;
mod transforming;
mod user_types;

fn main() {
    os::thread::map_reduce_example();
}

pub fn hierarchy() {
    inner::hello::hello();
    // inner::inner::hello();
}
