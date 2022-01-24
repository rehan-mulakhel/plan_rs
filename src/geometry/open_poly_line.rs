use crate::geometry::{point::Point, poly_line::PolyLine};

pub struct OpenPolyLine {
    points: Vec<Point>,
}

impl OpenPolyLine {
    pub fn new(points: Vec<Point>) -> OpenPolyLine {
        OpenPolyLine { points }
    }
}

impl PolyLine for OpenPolyLine {
    fn get_points(&self) -> &Vec<Point> {
        &self.points
    }

    fn is_closed(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_not_closed() {
        let points = vec![Point::new(0.0, 0.0)];
        let open_poly_line = OpenPolyLine::new(points);
        assert!(!open_poly_line.is_closed());
    }
}
