//! Stores the times of various watch points
//!

pub struct Watch<const SIZE: usize> {
    last_update_time: instant::Instant,
    update_time: instant::Instant,
    watch_points: [WatchPoint; SIZE],

    index_start: usize,
    index_end: usize, 
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
            index_start: 0,
            index_end: 0,
        }
    }

    pub fn start(&mut self, name: &'static str) {
        assert!(self.index_start < SIZE);

        if self.index_start != self.index_end {
            self.stop();
        }

        if self.index_start >= SIZE {
            return;
        }

        let now = instant::Instant::now();
        self.watch_points[self.index_start].start = now;
        self.watch_points[self.index_start].name = name;

        self.index_start += 1;
    }

    pub fn stop(&mut self) {
        assert!(self.index_start <= SIZE);
        
        if self.index_start != self.index_end+1 {
            return;
        }
        
        let now = instant::Instant::now();
        self.watch_points[self.index_end].stop = now;

        self.index_end = self.index_start;
    }

    pub fn update(&mut self) {
        let now = instant::Instant::now();
        self.last_update_time = self.update_time;
        self.update_time = now;

        self.index_start = 0;
        self.index_end = 0;
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

    pub fn get_viewer_data(&mut self) -> WatchViewerData<SIZE> {
        self.validate();
        WatchViewerData {
            last_update_time: self.last_update_time,
            update_time: self.update_time,
            watch_points: self.watch_points,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct WatchPoint {
    pub start: instant::Instant,
    pub stop: instant::Instant,
    pub name: &'static str,
}

impl WatchPoint {
    fn new(t: instant::Instant) -> Self {
        Self {
            start: t,
            stop: t,
            name: "",
        }
    }
}

#[derive(Clone, Debug)]
pub struct WatchViewerData<const SIZE: usize> {
    pub last_update_time: instant::Instant,
    pub update_time: instant::Instant,
    pub watch_points: [WatchPoint; SIZE],
}

impl<const SIZE: usize> Default for WatchViewerData<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> WatchViewerData<SIZE> {
    pub fn new() -> Self {
        let last_update_time = instant::Instant::now();
        let update_time = last_update_time;
        let watch_points = [WatchPoint::new(update_time); SIZE];

        Self {
            last_update_time,
            update_time,
            watch_points,
        }
    }
}
