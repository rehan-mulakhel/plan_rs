pub struct PointGeo {
    latitude: f64,
    longitude: f64,
}

impl PointGeo {
    pub const fn new(longitude: f64, latitude: f64) -> PointGeo {
        PointGeo {
            longitude: longitude,
            latitude: latitude,
        }
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn latitude() {
        for l in vec![-PI / 2.0, -PI / 4.0, 0.0, PI / 2.0] {
            assert_eq!(l, PointGeo::new(0.0, l).latitude());
        }
    }

    #[test]
    fn longitude() {
        for l in vec![-PI, -PI / 6.0, 0.0, PI / 2.0] {
            assert_eq!(l, PointGeo::new(l, 0.0).longitude());
        }
    }
}
