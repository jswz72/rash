extern crate rash;

fn main() {
    rash::load_config();
    rash::shell_loop();
    rash::shutdown();
}
