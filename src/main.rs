use viewport::run;
use primitive::add;

fn main() {
    add(2, 4);
    pollster::block_on(run());
}