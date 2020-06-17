pub struct ToyVec<T> {
    // T型の要素を格納する領域。各要素はヒープ領域に置かれる
    elements: Box<[T]>,
    // ベクタの長さ
    len: usize,
}

impl<T: Default> ToyVec<T> {
    // newはキャパシティ(容量)が0のToyVecを作る
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    // with_capacityは指定されたキャパシティを持つToyVecを作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size) // T型のデフォルト値をsize個作り
            .collect::<Vec<_>>() // Vec<T>に収集してから
            .into_boxed_slice() // Box<[T]>に変換する
    }

    pub fn len(&self) -> usize {
        self.len()
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }
}

fn main() {
    println!("Hello, world!");
}
