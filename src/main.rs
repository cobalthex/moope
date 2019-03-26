mod object;
mod runtime;
use std::time::Duration;

struct TestLogic {}
impl object::Logic for TestLogic
{
    fn name(&self) -> &'static str { return "TestLogic"; }
    fn Think(&mut self, dt : &object::FrameTime) -> Duration
    {
        println!("{}", dt.total.as_millis());
        return Duration::from_millis(10);
    }
}

fn main() {
    let mut runtime = runtime::Runtime::new();
    runtime.AddLogic(Box::new(TestLogic {}));

    loop {
        runtime.RunOnce();
    }
}
