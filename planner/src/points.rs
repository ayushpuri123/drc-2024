use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Default)] // needed for copy on DriveState, TODO: do i need Copy on DriveState
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn dist(&self, other: Pos) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn lerp(&self, other: Pos, t: f64) -> Pos {
        Pos {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    pub fn dist_along(&self, other: Pos, dist: f64) -> Pos {
        let t = dist / self.dist(other);
        self.lerp(other, t)
    }
}

pub enum PointType {
    LeftLine,
    RightLine,
    Obstacle,
    ArrowLeft,
    ArrowRight,
}

pub struct Point {
    pub pos: Pos,
    pub confidence: f64,
    pub point_type: PointType,
}

pub trait PointMap {
    fn get_points_in_area(&self, around: Pos, max_dist: f64) -> Vec<&Point>;
    fn get_points_below_confidence(&self, cutoff: f64) -> Vec<&Point>;
    fn get_points_lowest_confidence(&self, number: f64) -> Vec<&Point>;
    fn add_points(&mut self, points: &mut Vec<Point>);
}

pub struct SimplePointMap {
    all_points: Vec<Point>,
}

impl SimplePointMap {
    pub fn new() -> SimplePointMap {
        SimplePointMap {
            all_points: Vec::new(),
        }
    }
}

impl PointMap for SimplePointMap {
    fn get_points_in_area(&self, around: Pos, max_dist: f64) -> Vec<&Point> {
        self.all_points
            .iter()
            .filter(|point| point.pos.dist(around) < max_dist)
            .collect()
    }

    fn add_points(&mut self, points: &mut Vec<Point>) {
        self.all_points.append(points);
    }

    fn get_points_below_confidence(&self, cutoff: f64) -> Vec<&Point> {
        vec![]
    }

    fn get_points_lowest_confidence(&self, number: f64) -> Vec<&Point> {
        vec![]
    }
}

const GRID_SIZE: f64 = 0.2;

struct GridIndex {
    x: i16,
    y: i16,
}
pub struct GridPointMap {
    grid: HashMap<GridIndex, Vec<Point>>,
}

// impl PointMap for GridPointMap {
//     fn get_points(&self, around: Pos, max_dist: f64) -> Vec<Point>{

//     }
// }
