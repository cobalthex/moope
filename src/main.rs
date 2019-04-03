mod instance;
mod runtime;
mod graphics;

use std::time::Duration;


fn main()
{
    let mut runtime = runtime::Runtime::new();
    runtime.add_logic(graphics::WindowLogic::new(&mut runtime));

    let instance = runtime.new_instance();

    let mut window = runtime.get_logic::<graphics::WindowLogic>().unwrap().new_window(instance, "Window", 800.0, 600.0);

    loop {
        match runtime.run_once()
        {
            runtime::ShouldExit::Yes => break,
            _ => ()
        }
    }
}
