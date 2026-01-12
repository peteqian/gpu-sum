fn main() {
    pollster::block_on(run());
}

async fn run() {
    println!("Hello, world!");
}
