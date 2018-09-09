extern crate rush;

fn main() {
    let config = rush::initialize();
    rush::shell_loop(config);
    rush::shutdown();
}
