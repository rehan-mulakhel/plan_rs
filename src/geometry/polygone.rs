use crate::geometry::closed_poly_line::ClosedPolyLine;

pub struct Polygone {
    shell: ClosedPolyLine,
    holes: Vec<ClosedPolyLine>,
}

impl Polygone {
    fn holes(&self) -> &Vec<ClosedPolyLine> {
        &self.holes
    }

    pub fn new(shell: ClosedPolyLine, holes: Vec<ClosedPolyLine>) -> Polygone {
        Polygone { shell, holes }
    }

    fn shell(&self) -> &ClosedPolyLine {
        &self.shell
    }
}
