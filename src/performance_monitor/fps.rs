//! Tracks the frames per second

use instant::Duration;

pub struct Fps {
    duration : Duration,
    count: u32,
    fps: u32,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            duration: Duration::new(0, 0),
            count: 0,
            fps: 0,
        }
    }

    pub fn update(&mut self, dt: instant::Duration) 
    {
        self.duration += dt;
        self.count += 1;

        let one_sec = instant::Duration::new(1, 0);

        if self.duration >= one_sec {
            self.duration -= one_sec;
            self.fps = self.count;
            self.count = 0;
        }
    }

    pub fn get(&self) -> u32 {
        self.fps
    }
}

