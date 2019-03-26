use std::time::Duration;

pub type ObjectRef = usize;

pub struct Object
{
    pub refID : ObjectRef,
    pub name : Option<String>
}

impl Object
{

}

pub struct FrameTime
{
    pub delta : Duration,
    pub total : Duration,
}

pub trait Logic
{
    fn name(&self) -> &'static str;

    fn Think(&mut self, time : &FrameTime) -> Duration; // returns next think time (from now)
}