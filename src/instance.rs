use runtime::*;

use std::time::Duration;

pub type Instance = usize;

pub trait Logic
{
    fn name(&self) -> &'static str;

    fn init(&self, _runtime : &mut Runtime) { }
    fn think(&mut self, rops : &mut RuntimeOps, time : &FrameTime) -> Duration; // returns next think time (from now)

    fn attach(&mut self, instance : Instance);
    fn detach(&mut self, instance : Instance);
}

