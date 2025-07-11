/// Represents an x-y coordinate.
#[derive(Debug, Clone)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

#[cfg(target_os = "linux")]
impl From<Point> for smithay::utils::Point<f64, smithay::utils::Logical> {
  fn from(point: Point) -> Self {
    smithay::utils::Point::new(f64::from(point.x), f64::from(point.y))
  }
}

#[cfg(target_os = "linux")]
#[allow(clippy::cast_possible_truncation)]
impl From<smithay::utils::Point<f64, smithay::utils::Logical>> for Point {
  fn from(
    point: smithay::utils::Point<f64, smithay::utils::Logical>,
  ) -> Self {
    Point {
      x: point.x as i32,
      y: point.y as i32,
    }
  }
}
