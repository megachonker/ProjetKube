mod viewport;
mod primitive;

pub use crate::viewport::run;
pub use crate::primitive::test;


fn main() {


    // print the mesh
    test();
    // pollster::block_on(run());
}