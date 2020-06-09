// --- 5-3-1 型エイリアス
type UserName = String;
type Id = i64;
type Timestamp = i64;
type User = (Id, UserName, Timestamp);

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

// --- 5-3-2 構造体(struct)
// 名前付きフィールド構造体

// #[derive(Default)] // implブロックを自前実装しない場合
struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

// Defaultを自動導出するには構造体のすべてのフィールドの型がDefaultトレイトを実装している必要がある
// 自動導出できない場合や、自動導出とは異なるデフォルト値を持たせたい場合、
// implブロックでDefaultトレイトを実装する
impl Default for Polygon {
    fn default() -> Self {
        Self {
            stroke_width: 1,
            vertexes: Default::default(),
            fill: Default::default(),
        }
    }
}

// タプル構造体
struct Triangle(Vertex, Vertex, Vertex);
struct Vertex(i32, i32);

// newtype: 型エイリアスの代わりにフィールドが1つのタプル構造体を使うデザインパターン
// struct Id(u64);

// ユニット構造体
struct UniqueValue;
// または
// struct UniqueValue {}
// struct UniqueValue();

// Rust 1.17で導入されたフィールド初期化略記構文
// フィールド名と同じ名前の関数引数やローカル変数があるときは省略形が使える
fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
    let stroke_width = 1;
    let fill = (0, 0, 0);
    Polygon {
        vertexes,
        stroke_width,
        fill,
    }
}

fn main() {
    // --- 5-3-1 型エイリアス
    let id = 400;
    let now = 4567890123;
    let user = new_user(String::from("mika"), id, now);
    println!("{:?}", user);

    // --- 5-3-2 構造体(struct)
    // 1. 名前付きフィールド構造体
    let triangle = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 1,
    };

    let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);

    // フィールドへのアクセス: フィールド名でアクセス
    assert_eq!(triangle.vertexes[0], (0, 0));
    assert_eq!(triangle.vertexes.len(), 3);

    // フィールドへのアクセス: パターンマッチでアクセス
    let Polygon {
        vertexes: quad_vx, ..
    } = quadrangle;
    assert_eq!(4, quad_vx.len());

    // :以降を省略すると、フィールドと同じ名前の変数が作られフィールド値に束縛される
    let Polygon { fill, .. } = quadrangle;
    assert_eq!((0, 0, 0), fill);

    // 構造体の値を変更するにはmutが必要
    let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    assert_eq!(polygon.vertexes.len(), 2);
    polygon.vertexes.push((2, 8));
    assert_eq!(polygon.vertexes.len(), 3);

    // 既存の値から新しい値を作る
    // (関数型レコードアップデート構文
    let triangle2 = Polygon {
        vertexes: vec![(0, 0), (-3, 0), (-2, 2)],
        ..triangle
    };

    // 2. タプル構造体
    let vx0 = Vertex(0, 0);
    let vx1 = Vertex(3, 0);
    let triangle = Triangle(vx0, vx1, Vertex(2, 2));

    assert_eq!((triangle.1).0, 3);
}
