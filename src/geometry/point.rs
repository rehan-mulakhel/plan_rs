pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_getter() {
        for x_int in (-100..100).step_by(15) {
            let x = f64::from(x_int);
            assert_eq!(x, Point::new(x, 0.0).x());
        }
    }

    #[test]
    fn y_getter() {
        for y_int in (-100..100).step_by(15) {
            let y = f64::from(y_int);
            assert_eq!(y, Point::new(0.0, y).y());
        }
    }
}
