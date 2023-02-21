mod viewport;
mod primitive;

pub use crate::viewport::run;
pub use crate::primitive::test;

fn main() {
    test();
    // pollster::block_on(run());
}