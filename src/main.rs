use crate::scripting::run_engine;

mod scripting;

fn main() {
    println!("Booting game engine.");
    let _ = run_engine();
}
