use std::{thread, time};

fn main() {
    let n1 = 1200;
    let n2 = 1000;

    // spawnで子スレッドを立ち上げ、子スレッドで重い処理を実行する
    // 変数childがスレッドへのハンドルに束縛される
    let child = thread::spawn(move || {
        // 重い処理を実行する
        heavy_calc("child", n2)
    });

    // 親スレッドでも重い処理を実行する。子スレッドの処理と同時に実行される
    let s1 = heavy_calc("main", n1);

    // スレッドのハンドルに対してjoinを呼ぶことで、スレッドの終了を待つ
    // クロージャの戻り値はOkでラップされる。もしスレッドがエラーにより
    // 異常終了したならErrが返る
    match child.join() {
        Ok(s2) => println!("{}, {}", s1, s2),
        Err(e) => println!("error: {:?}", e),
    }
}

fn heavy_calc(name: &str, n: u64) -> u64 {
    println!("{}: started.", name);
    // 重い処理の代用としてnミリ秒スリープする
    thread::sleep(time::Duration::from_millis(n));
    // 1からnまでの数字を足し合わせる
    let sum = (1..=n).sum();
    println!("{}: ended.", name);
    sum
}
