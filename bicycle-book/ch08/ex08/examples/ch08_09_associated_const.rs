// デカルト座標
struct CartesianCoord {
  x: f64,
  y: f64,
}

trait Dimension {
  const DIMENSION: u32;
}

impl Dimension for CartesianCoord {
  const DIMENSION: u32 = 2;
}

fn main() {
  let dim = CartesianCoord::DIMENSION;
  println!("{}", dim);
}
