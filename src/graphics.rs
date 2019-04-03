use instance::Instance;
use std::collections::HashMap;
use runtime::Runtime;
use std::time::Duration;
extern crate winit;

use instance;
use runtime;

pub struct Window
{
    window : winit::Window,
    fps : u32, //fps of most recent frame
}

pub struct WindowLogic<'a>
{
    runtime : &'a mut Runtime,

    windows : HashMap<Instance, Window>,
    window_handles: HashMap<winit::WindowId, Instance>,
    events_loop : winit::EventsLoop,
}
impl<'a> WindowLogic<'a>
{
    pub fn new(runtime : &'a mut Runtime) -> Self
    {
        let events_loop = winit::EventsLoop::new();
        return WindowLogic
        {
            runtime: &mut runtime,
            windows: HashMap::new(),
            window_handles: HashMap::new(),
            events_loop: events_loop
        };
    }

    pub fn new_window(&mut self, instance : Instance, title: &str, width: f64, height: f64) -> Window
    {
        let native = winit::Window::new(&self.events_loop).unwrap();
        native.set_title(title);
        native.set_inner_size(winit::dpi::LogicalSize{width: width, height: height});
        native.set_resizable(false);

        let window = Window { window: native, fps: 0 };

        self.windows[&instance] = window;
        return window;
    }

    pub fn get_native_window(&self, instance : Instance) -> Option<&mut winit::Window>
    {
        match self.windows.get_mut(&instance)
        {
            Some(w) => Some(&mut w.window),
            None => None
        }
    }

    pub fn close_window(&mut self, window_id : winit::WindowId)
    {
        match self.window_handles.get(&window_id)
        {
            Some(i) =>
            {
                self.windows.remove(&i);
                self.window_handles.remove(&window_id);
            }
            _ => {}
        }
    }
}
impl<'a> instance::Logic for WindowLogic<'a>
{
    fn name(&self) -> &'static str { return "Window"; }
    fn think(&mut self, rops : &mut runtime::RuntimeOps, dt : &runtime::FrameTime) -> Duration
    {
        self.events_loop.poll_events(|event|
        {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    window_id: wid
                } =>
                {
                    self.close_window(wid);
                    if self.windows.is_empty() {
                        rops.exit = runtime::ShouldExit::Yes
                    }
                },
                _ => ()
            };
        });

        //self.fps = u32::pow(10, 9) / dt.delta.as_nanos() as u32;
        return Duration::from_millis(0);
    }

    fn attach(&mut self, inst : Instance)
    {
        self.new_window(inst, "Window", 800.0, 600.0);
    }

    fn detach(&mut self, inst : Instance)
    {
        match self.windows.get(&inst)
        {
            Some(w) =>
            {
                self.window_handles.remove(&w.window.id());
                self.windows.remove(&inst);
            }
            _ => {}
        }
    }
}

