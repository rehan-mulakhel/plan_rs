use super::{open_poly_line::OpenPolyLine, point::Point};

pub trait PolyLine {
    fn first_point(&self) -> &Point {
        &self.get_points()[0]
    }
    fn get_points(&self) -> &Vec<Point>;
    fn is_closed(&self) -> bool;
}
