/// Option


// 部分的に定義された構造体型
struct BagOfHolding<T> {
    // パラメータ T を渡すことが可能
    item: Option<T>,
}

fn main() {
    // 注意: i32 が入るバッグに、何も入っていません！
    // None からは型が決められないため、型を指定する必要があります。
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("バッグには何もない！")
    } else {
        println!("バッグには何かある！")
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };

    if i32_bag.item.is_some() {
        println!("バッグには何かある！")
    } else {
        println!("バッグには何もない！")
    }

    // match は Option をエレガントに分解して、
    // すべてのケースが処理されることを保証できます！
    match i32_bag.item {
        Some(v) => println!("バッグに {} を発見！", v),
        None => println!("何も見付からなかった"),
    }
}
