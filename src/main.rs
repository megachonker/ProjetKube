mod viewport;
pub use crate::viewport::run;

fn main() {
    pollster::block_on(run());
}