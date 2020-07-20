use std::io;
use std::io::Write;

fn main() {
    let mut year = String::new();
    print!("Please input a year to check if it is a leap year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    let year = year.trim().parse::<u32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year!", year);
    } else {
        println!("{} is not a leap year.", year);
    }

    // --- 6-5-3 メソッド, 6-5-4 関連関数
    let circle1 = Circle { radius: 10 };
    println!("Circle1's diameter: {}", circle1.diameter());
    let circle1 = Circle::small_circle();
    println!("Circle1's diameter: {}", circle1.diameter());

    let some_point = Point { x: 10, y: 20, z: 0 };
    println!("Debug: {:?}", some_point);
}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}

// --- 6-5-3 メソッド, 6-5-4 関連関数
struct Circle {
    radius: u32,
}

impl Circle {
    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    fn small_circle() -> Circle {
        Circle { radius: 1 }
    }
}

// --- 6-11 アトリビュート
// cfgアトリビュート: 条件によってコンパイルするかどうかを決めることができる
#[cfg(unix)]
fn somthing_for_unix() {}

#[cfg(windows)]
fn somthing_for_windows() {}

// deriveアトリビュート: 対応したトレイトの実装を、自動的に構造体や列挙型に実装してもらえる
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
