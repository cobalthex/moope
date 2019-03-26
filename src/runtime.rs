use std::collections::HashMap;
use std::time::{Duration, Instant};

use object::*;

struct LogicEntry
{
    pub logic : Box<dyn Logic>,
    pub nextThink : Instant
}

pub struct Runtime
{
    lastTick: Instant,
    elapsed: Duration,
    objects : HashMap<ObjectRef, Object>,
    logics : Vec<LogicEntry>
}

impl Runtime
{
    pub fn new() -> Self
    {
        return Runtime
        {
            lastTick: Instant::now(),
            elapsed: Duration::new(0, 0),
            objects: HashMap::new(),
            logics: Vec::new()
        };
    }

    pub fn AddLogic(&mut self, logic : Box<dyn Logic>)
    {
        self.logics.push(LogicEntry { logic: logic, nextThink: Instant::now() });
    }

    pub fn RunOnce(&mut self)
    {
        let nextTick = Instant::now();
        let delta = nextTick - self.lastTick;
        self.lastTick = nextTick;
        self.elapsed += delta;
        let dt = FrameTime
        {
            delta: delta,
            total: self.elapsed,
        };

        for logic in self.logics.iter_mut()
        {
            if self.lastTick >= logic.nextThink
            {
                logic.nextThink = self.lastTick + logic.logic.Think(&dt);
            }
        }
    }
}