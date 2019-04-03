use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::any::{Any, TypeId};

use instance::*;

struct LogicEntry
{
    pub next_think : Instant,
    pub logic : Box<dyn Logic>,
}

pub struct Runtime
{
    last_tick : Instant,
    elapsed : Duration,
    instance_names : HashMap<Instance, Option<String>>,
    instance_counter : usize,
    logics : HashMap<TypeId, LogicEntry>
}

pub struct FrameTime
{
    pub delta : Duration,
    pub total : Duration,
}

pub enum ShouldExit { Yes, No }

pub struct RuntimeOps
{
    pub exit: ShouldExit,

}

impl<'a> Runtime
{
    #[inline]
    pub fn new() -> Self
    {
        return Runtime
        {
            last_tick: Instant::now(),
            elapsed: Duration::new(0, 0),
            instance_names: HashMap::new(),
            instance_counter: 0,
            logics: HashMap::new()
        };
    }

    #[inline]
    pub fn add_logic<T: 'static + Logic>(&mut self, logic : T)/* why the fuck does this need to be static */
    {
        self.logics.insert(TypeId::of::<T>(), LogicEntry { logic: Box::new(logic), next_think: Instant::now() });
        //return self.get_logic<T>(false);
    }

    pub fn get_logic<T: 'static + Logic>(&mut self) -> Option<&mut T>
    {
        let tid = TypeId::of::<T>();
        match self.logics.get_mut(&tid)
        {
            Some(mut v) =>
            {
                Some(&*v.logic as &mut T)
            },
            None => None
        }
    }

    pub fn new_instance(&mut self) -> Instance
    {
        self.instance_counter += 1;
        self.instance_names[&self.instance_counter] = None;
        return self.instance_counter;
    }

    pub fn destroy_instance(&mut self, instance : Instance)
    {
        self.instance_names.remove(&instance);
    }

    pub fn run_once(&mut self) -> ShouldExit
    {
        let next_tick = Instant::now();
        let delta = next_tick - self.last_tick;
        self.last_tick = next_tick;
        self.elapsed += delta;
        let dt = FrameTime
        {
            delta: delta,
            total: self.elapsed,
        };

        let mut rops = RuntimeOps
        {
            exit: ShouldExit::No
        };

        for (_, logic) in self.logics.iter_mut()
        {
            if self.last_tick >= logic.next_think
            {
                logic.next_think = self.last_tick + logic.logic.think(&mut rops, &dt);
            }
        }

        return rops.exit;
    }
}