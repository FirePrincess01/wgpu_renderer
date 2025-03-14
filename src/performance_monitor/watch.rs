//! Stores the times of various watchpoints
//!

#[derive(Copy, Clone)]
pub struct WatchPoint {
    pub start: instant::Instant,
    pub stop: instant::Instant,
    pub name: &'static str,
}

impl WatchPoint {
    fn new(t: instant::Instant) -> Self {
        Self { start: t, stop: t, name: "" }
    }
}

pub struct Watch<const SIZE: usize> {
    last_update_time: instant::Instant,
    update_time: instant::Instant,
    watch_points: [WatchPoint; SIZE],
}

pub trait Viewer {
    fn update(
        &mut self,
        last_update_time: instant::Instant,
        update_time: instant::Instant,
        watch_points: &[WatchPoint],
    );
}

impl<const SIZE: usize> Default for Watch<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> Watch<SIZE> {
    pub fn new() -> Self {
        let now = instant::Instant::now();
        let last_update_time = now;
        let update_time = now;
        let watch_points = [WatchPoint::new(now); SIZE];

        Self {
            last_update_time,
            update_time,
            watch_points,
        }
    }

    pub fn start(&mut self, index: usize, name: &'static str) {
        if index < SIZE {
            let now = instant::Instant::now();
            self.watch_points[index].start = now;
            self.watch_points[index].name = name;
        }
    }

    pub fn stop(&mut self, index: usize) {
        if index < SIZE {
            let now = instant::Instant::now();
            self.watch_points[index].stop = now;
        }
    }

    pub fn update(&mut self) {
        let now = instant::Instant::now();
        self.last_update_time = self.update_time;
        self.update_time = now;
    }

    fn validate(&mut self) {
        for i in 0..SIZE {
            if self.watch_points[i].start > self.watch_points[i].stop
                || self.watch_points[i].stop > self.update_time
                || self.watch_points[i].start > self.update_time
            {
                self.watch_points[i].stop = self.update_time;
                self.watch_points[i].start = self.update_time;
            }
        }
    }

    pub fn update_viewer(&mut self, viewer: &mut impl Viewer) {
        self.validate();
        viewer.update(self.last_update_time, self.update_time, &self.watch_points);
    }
}
