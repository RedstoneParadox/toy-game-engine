use crate::scripting::run_engine;

mod scripting;
mod game_object;

fn main() {
    println!("Booting game engine.");
    let _ = run_engine();
}
