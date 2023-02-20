mod viewport;
mod primitive;

pub use crate::viewport::run;
pub use crate::primitive::add;

fn main() {
    add(2, 4);
    pollster::block_on(run());
}