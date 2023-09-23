fn main() {
  let x = 42;

  match x {
      0 => {
          println!("found zero");
      }
      // 複数の値にマッチ
      1 | 2 => {
          println!("found 1 or 2!");
      }
      // 範囲にマッチ
      3..=9 => {
          println!("found a number 3 to 9 inclusively");
      }
      // マッチした数字を変数に束縛
      matched_num @ 10..=100 => {
          println!("found {} number between 10 to 100!", matched_num);
      }
      // どのパターンにもマッチしない場合のデフォルトマッチが必須
      _ => {
          println!("found something else!");
      }
  }
}
