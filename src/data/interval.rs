#[derive(Clone, Copy)]
pub struct Interval {
    start: f64,
    end: f64,
}

impl Interval {
    pub const fn new(start: f64, end: f64) -> Interval {
        Interval { start, end }
    }

    pub const fn empty() -> Interval {
        Interval { start: f64::INFINITY, end: f64::NEG_INFINITY }
    }

    pub const fn start(&self) -> f64 {
        self.start
    }

    pub const fn end(&self) -> f64 {
        self.end
    }

    pub const fn size(&self) -> f64 {
        self.end - self.start
    }

    pub const fn contains(&self, x: f64) -> bool {
        self.start <= x && x <= self.end
    }

    pub const fn surrounds(&self, x: f64) -> bool {
        self.start < x && x < self.end
    }
}

const EMPTY: Interval = Interval::empty();
const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);