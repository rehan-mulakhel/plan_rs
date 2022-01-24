use crate::geometry::{point::Point, poly_line::PolyLine};
use std::convert::TryInto;

pub struct ClosedPolyLine {
    points: Vec<Point>,
}

impl ClosedPolyLine {
    pub fn area(&self) -> f64 {
        let mut acc = 0.0;
        for i in 0..self.points.len() {
            let p_i = &self.points[self.idx(i as i32)];
            let p_before = &self.points[self.idx((i as i32) - 1)];
            let p_after = &self.points[self.idx((i as i32) + 1)];
            acc += p_i.x() * (p_after.y() - p_before.y());
        }
        acc.abs() / 2.0
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        let mut acc = 0;
        for i in 0..self.points.len() {
            let p1 = &self.points[i];
            let p2 = &self.points[(self.idx((i as i32) + 1))];
            if p1.y() <= point.y() {
                if p2.y() > point.y() && self.on_leftside(point, p1, p2) {
                    acc += 1;
                }
            } else {
                if p2.y() <= point.y() && self.on_leftside(point, p2, p1) {
                    acc -= 1;
                }
            }
        }
        acc != 0
    }

    pub fn new(points: Vec<Point>) -> ClosedPolyLine {
        ClosedPolyLine { points }
    }

    fn idx(&self, i: i32) -> usize {
        if i < 0 {
            self.points.len() - 1
        } else if i >= self.points.len().try_into().unwrap() {
            0
        } else {
            i as usize
        }
    }

    fn on_leftside(&self, p: &Point, p1: &Point, p2: &Point) -> bool {
        (p1.x() - p.x()) * (p2.y() - p.y())
            > (p2.x() - p.x()) * (p1.y() - p.y())
    }
}

impl PolyLine for ClosedPolyLine {
    fn get_points(&self) -> &Vec<Point> {
        &self.points
    }

    fn is_closed(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let open_poly_line = ClosedPolyLine::new(vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.0, 10.0),
        ]);
        assert!((open_poly_line.area() - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn contains_point() {
        let open_poly_line = ClosedPolyLine::new(vec![
            Point::new(1.0, 0.0),
            Point::new(5.0, 0.0),
            Point::new(4.0, 2.0),
            Point::new(5.0, 5.0),
            Point::new(5.0, 5.0),
        ]);
        assert!(open_poly_line.contains_point(&Point::new(3.0, 1.0)));
        assert!(!open_poly_line.contains_point(&Point::new(5.0, 2.5)));
    }

    #[test]
    fn is_closed() {
        let points = vec![Point::new(0.0, 0.0)];
        let open_poly_line = ClosedPolyLine::new(points);
        assert!(open_poly_line.is_closed());
    }
}
