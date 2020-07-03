mod coord;

use crate::coord::print_point;
use crate::coord::Coordinates;

fn main() {
    let p = (1.0, 0.0).to_cartesian();
    print_point(p);
}
