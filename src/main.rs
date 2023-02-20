use kube::run;

fn main() {
    pollster::block_on(run());
}