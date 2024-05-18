pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        (self.min..=self.max).contains(&x)
    }

    pub fn surrounds(&self, x: f64) -> bool {
        (self.min < x) && (x < self.max)
    }

    pub fn empty() -> Self {
        Interval {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self::default()
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: -f64::INFINITY,
            max: -f64::INFINITY,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interval::Interval;

    #[test]
    fn test_contains() {
        let interval = Interval { min: -1., max: 1. };
        assert_eq!(interval.contains(-0.5), true);
        assert_eq!(interval.contains(-1.5), false);
    }

    #[test]
    fn test_surrounds() {
        let interval = Interval { min: -1., max: 1. };
        assert_eq!(interval.surrounds(-0.5), true);
        assert_eq!(interval.surrounds(-1.), false);
    }
}
