mod engine;

use engine::main::engine::Engine;

fn main() -> Result<(), String>{

    let mut engine: Engine = Engine::new()?;
    
    unsafe {
        engine.start();
    }

    Ok(())
}
