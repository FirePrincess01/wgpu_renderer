//! Stores the times of various watchpoints
//!

#[derive(Copy, Clone)]
pub struct Watchpoint
{
    pub start: instant::Instant,
    pub stop: instant::Instant,
}

impl Watchpoint {

    fn new(t: instant::Instant) -> Self {
        Self {
            start: t,
            stop: t,
        }
    }
}

pub struct Watch<const SIZE:usize>
{
    last_update_time: instant::Instant,
    update_time: instant::Instant,
    watchpoints: [Watchpoint; SIZE],
}

pub trait Viewer {
    fn update(&mut self, last_update_time: instant::Instant, update_time: instant::Instant, watchpoints: &[Watchpoint]);
}


impl <const SIZE:usize> Watch<SIZE>{

    pub fn new() -> Self {
        let now = instant::Instant::now();        
        let last_update_time = now;
        let update_time = now;
        let watchpoints = [Watchpoint::new(now); SIZE];

        Self {
            last_update_time,
            update_time,
            watchpoints,
        }
    }

    pub fn start(&mut self, index: usize) {
        if index < SIZE {
            let now = instant::Instant::now();
            self.watchpoints[index].start = now;
        }
    }

    pub fn stop(&mut self, index: usize) {
        if index < SIZE {
            let now = instant::Instant::now();
            self.watchpoints[index].stop = now;
        }
    }

    pub fn update(&mut self) {
        let now = instant::Instant::now();
        self.last_update_time = self.update_time;
        self.update_time = now;
    }

    fn validate(&mut self) {
        for i in 0..SIZE {
            if self.watchpoints[i].start > self.watchpoints[i].stop ||
               self.watchpoints[i].stop  > self.update_time ||
               self.watchpoints[i].start > self.update_time  
            {
                self.watchpoints[i].stop = self.update_time;
                self.watchpoints[i].start = self.update_time;
            }
        }
    }

    pub fn update_viewer(&mut self, viewer: &mut impl Viewer) {
        self.validate();
        viewer.update(self.last_update_time, self.update_time, &self.watchpoints);
    }

}