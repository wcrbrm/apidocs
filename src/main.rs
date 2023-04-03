pub mod logging;

fn main() {
    color_eyre::install().unwrap();
    logging::start();

    println!("Hello, world!");
}
