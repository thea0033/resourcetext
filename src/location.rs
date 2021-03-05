#[derive(Clone, Debug, Copy, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub x: f64,
    pub y: f64,
}
impl Location {
    pub fn new(x: f64, y: f64) -> Location {
        Location { x, y }
    }
    pub fn close_enough(&self, other: &Location, required_distance: f64) -> bool {
        let calculation: f64 = ((self.x - other.x).abs().powf(2.0) + (self.y - other.y).abs().powf(2.0)).sqrt();
        calculation <= required_distance
    }
    pub fn eq(&self, other: &Location) -> bool {
        self.x == other.x && self.y == other.y
    }
    pub fn move_towards(&mut self, other: Location, distance: f64) {
        let total_dist = f64::sqrt((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y));
        let mut ratio = distance / total_dist;
        if ratio > 1.0 {
            ratio = 1.0;
        }
        self.x = self.x * (1.0 - ratio) + other.x * ratio;
        self.y = self.y * (1.0 - ratio) + other.y * ratio;
    }
}
