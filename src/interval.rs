pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn contains(&self, x: f32) -> bool {
        (self.min..=self.max).contains(&x)
    }

    pub fn surrounds(&self, x: f32) -> bool {
        (self.min < x) && (x < self.max)
    }

    pub fn empty() -> Self {
        Interval {
            min: f32::INFINITY,
            max: -f32::INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self::default()
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: -f32::INFINITY,
            max: -f32::INFINITY,
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
