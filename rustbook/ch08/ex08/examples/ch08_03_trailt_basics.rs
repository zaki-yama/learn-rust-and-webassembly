// デカルト座標
struct CartesianCoord {
  x: f64,
  y: f64,
}

// 極座標
struct PolarCoord {
  r: f64,
  theta: f64,
}

trait Coordinates {
  fn to_cartesian(self) -> CartesianCoord;
  fn from_cartesian(cart: CartesianCoord) -> Self;
}

impl Coordinates for CartesianCoord {
  fn to_cartesian(self) -> CartesianCoord {
    self
  }

  fn from_cartesian(cart: CartesianCoord) -> Self {
    cart
  }
}

impl Coordinates for PolarCoord {
  fn to_cartesian(self) -> CartesianCoord {
    CartesianCoord {
      x: self.r * self.theta.cos(),
      y: self.r * self.theta.sin(),
    }
  }

  fn from_cartesian(cart: CartesianCoord) -> Self {
    Self {
      r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
      theta: (cart.y / cart.x).atan(),
    }
  }
}

// タプルにもトレイトを実装できる
impl Coordinates for (f64, f64) {
  fn to_cartesian(self) -> CartesianCoord {
    CartesianCoord {
      x: self.0,
      y: self.1,
    }
  }

  fn from_cartesian(cart: CartesianCoord) -> Self {
    (cart.x, cart.y)
  }
}

fn print_point(point: impl Coordinates) {
  let p = point.to_cartesian();
  println!("({}, {})", p.x, p.y);
}

struct Matrix([[f64; 2]; 2]);

trait LinearTransform: Coordinates {
  fn transform(self, matrix: &Matrix) -> Self;
}

impl LinearTransform for CartesianCoord {
  fn transform(mut self, matrix: &Matrix) -> Self {
    let x = self.x;
    let y = self.y;
    let m = matrix.0;

    self.x = m[0][0] * x + m[0][1] * y;
    self.y = m[1][0] * x + m[1][1] * y;
    self
  }
}

fn main() {
  let point = (1.0, 1.0);

  let c = point.to_cartesian();
  println!("x = {}, y = {}", c.x, c.y);

  let p = PolarCoord::from_cartesian(c);
  println!("r = {}, θ = {}", p.r, p.theta);

  print_point((0.0, 1.0));
  print_point(p);
  // print_point("string"); // error
}
