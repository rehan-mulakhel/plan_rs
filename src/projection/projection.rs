use crate::geometry::point::Point;
use crate::geometry::point_geo::PointGeo;

pub enum Projection {
    Ch1903,
    Equirectangular,
}

impl Projection {
    fn inverse(&self, p: &Point) -> PointGeo {
        use Projection::*;

        match *self {
            Ch1903 => {
                let x1 = (p.x() - 600000.0) / 1000000.0;
                let y1 = (p.y() - 200000.0) / 1000000.0;
                let lambda0 = 2.6779094
                    + 4.728982 * x1
                    + 0.791484 * x1 * y1
                    + 0.1306 * x1 * y1 * y1
                    - 0.0436 * x1 * x1 * x1;
                let phi0 = 16.9023892 + 3.238272 * y1
                    - 0.270978 * x1 * x1
                    - 0.002528 * y1 * y1
                    - 0.0447 * x1 * x1 * y1
                    - 0.0140 * y1 * y1 * y1;
                let latitude = phi0 * 100.0 / 36.0;
                let longitude = lambda0 * 100.0 / 36.0;
                PointGeo::new(longitude.to_radians(), latitude.to_radians())
            }
            Equirectangular => PointGeo::new(p.x(), p.y()),
        }
    }

    fn project(&self, p: &PointGeo) -> Point {
        use Projection::*;

        match *self {
            Ch1903 => {
                let l1 =
                    (p.longitude().to_degrees() * 3600.0 - 26782.5) / 10000.0;
                let p1 =
                    (p.latitude().to_degrees() * 3600.0 - 169028.66) / 10000.0;
                let x = 600072.37 + 211455.93 * l1
                    - 10938.51 * l1 * p1
                    - 0.36 * l1 * p1 * p1
                    - 44.54 * l1 * l1 * l1;
                let y = 200147.07
                    + 308807.95 * p1
                    + 3745.25 * l1 * l1
                    + 76.63 * p1 * p1
                    - 194.56 * l1 * l1 * p1
                    + 119.79 * p1 * p1 * p1;
                Point::new(x, y)
            }
            Equirectangular => Point::new(p.longitude(), p.latitude()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELTA: f64 = 0.000001;

    #[test]
    fn inverse_ch1903() {
        let point = PointGeo::new(0.115786678, 0.811730504);
        let projection = Projection::Ch1903.project(&point);
        let point_back = Projection::Ch1903.inverse(&projection);
        assert!((0.115786678 - point_back.longitude()) < DELTA);
        assert!((0.811730504 - point_back.latitude()) < DELTA);
    }

    #[test]
    fn inverse_equirectangular() {
        let point = PointGeo::new(0.115786678, 0.811730504);
        let projection = Projection::Equirectangular.project(&point);
        let point_back = Projection::Equirectangular.inverse(&projection);
        assert!((0.115786678 - point_back.longitude()) < DELTA);
        assert!((0.811730504 - point_back.latitude()) < DELTA);
        assert!((projection.x() - point_back.longitude()) < DELTA);
        assert!((projection.y() - point_back.latitude()) < DELTA);
    }
}
