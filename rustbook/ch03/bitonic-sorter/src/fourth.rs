use super::SortOrder;
use rayon;
use std::cmp::Ordering;

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
  F: Fn(&T, &T) -> Ordering,
{
  if x.len().is_power_of_two() {
    do_sort(x, true, comparator);
    Ok(())
  } else {
    Err(format!(
      "The length of x is not a power of two. (x.len(): {})",
      x.len()
    ))
  }
}

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
  match *order {
    SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
    SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
  }
}

const PARALLEL_THRESHOLD: usize = 4096;

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
  F: Fn(&T, &T) -> Ordering,
{
  if x.len() > 1 {
    let mid_point = x.len() / 2;

    // xの分割後の要素数をしきい値(PARALLEL_THRESHOLD)と比較する
    if mid_point >= PARALLEL_THRESHOLD {
      // しきい値以上なら並列にソートする(並列処理)
      rayon::join(
        || do_sort(&mut x[..mid_point], true, comparator),
        || do_sort(&mut x[mid_point..], false, comparator),
      );
    } else {
      do_sort(&mut x[..mid_point], true, comparator);
      do_sort(&mut x[mid_point..], false, comparator);
    }
    sub_sort(x, forward, comparator);
  }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
  F: Fn(&T, &T) -> Ordering,
{
  if x.len() > 1 {
    compare_and_swap(x, forward, comparator);
    let mid_point = x.len() / 2;
    sub_sort(&mut x[..mid_point], forward, comparator);
    sub_sort(&mut x[mid_point..], forward, comparator);
  }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
  F: Fn(&T, &T) -> Ordering,
{
  // 比較に先立ちforward(bool値)をOrdering値に変換しておく
  let swap_condition = if forward {
    Ordering::Greater
  } else {
    Ordering::Less
  };
  let mid_point = x.len() / 2;
  for i in 0..mid_point {
    // comparatorクロージャで2要素を比較し、返されたOrderingのバリアントが
    // swap_conditionと等しいなら要素を交換する
    if comparator(&x[i], &x[mid_point + i]) == swap_condition {
      // 要素を交換する
      x.swap(i, mid_point + i);
    }
  }
}

// このモジュールはcargo testを実行したときのみコンパイルされる
#[cfg(test)]
mod tests {
  // 親モジュール(first)のsort関数を使用する
  use super::{sort, sort_by};
  use crate::utils::{is_sorted_ascending, is_sorted_descending, new_u32_vec};
  use crate::SortOrder::*;

  // #[test]の付いた関数はcargo testとしたときに実行される
  #[test]
  fn sort_u32_ascending() {
    // テストデータとしてu32型のベクタを作成しxに束縛する
    // sort関数によって内容が更新されるので、可変を表すmutキーワードが必要
    let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

    // xのスライスを作成し、sort関数を呼び出す
    // &mut xは&mut x[..]と書いてもいい
    assert_eq!(sort(&mut x, &Ascending), Ok(()));

    // xの要素が昇順にソートされていることを確認する
    assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
  }

  #[test]
  fn sort_u32_descending() {
    let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
    assert_eq!(sort(&mut x, &Descending), Ok(()));

    // xの要素が降順にソートされていることを確認する
    assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
  }

  #[test]
  fn sort_str_ascending() {
    // 文字列のベクタを作り、ソートする
    let mut x = vec![
      "Rust",
      "is",
      "fast",
      "and",
      "memory-efficient",
      "with",
      "no",
      "GC",
    ];
    assert_eq!(sort(&mut x, &Ascending), Ok(()));
    assert_eq!(
      x,
      vec![
        "GC",
        "Rust",
        "and",
        "fast",
        "is",
        "memory-efficient",
        "no",
        "with"
      ]
    );
  }

  #[test]
  fn sort_to_fail() {
    let mut x = vec![10, 30, 11];
    assert!(sort(&mut x, &Ascending).is_err());
  }

  // #[test]
  // fn sort_f64() {
  //   // f64型の値をソートしたい
  //   let mut x = vec![20.0, -30.0, 11.0, 10.0]; // Vec<f64>型
  //   sort(&mut x, &Ascending);
  // }
  // 構造体Studentを定義する
  // 構造体は関連する値を1つにまとめたデータ構造。複数のデータフィールドを持つ
  #[derive(Debug, PartialEq)]
  struct Student {
    first_name: String,
    last_name: String,
    age: u8,
  }

  // implブロックを使うと対象の型に関連関数やメソッドを実装できる
  impl Student {
    // 関連関数newを定義する
    fn new(first_name: &str, last_name: &str, age: u8) -> Self {
      // 構造体Studentを初期化して返す。Selfはimpl対象の型(Student)の別名
      Self {
        // to_stringメソッドで&str型の引数からString型の値を作る
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        age,
      }
    }
  }

  #[test]
  // 年齢で昇順にテストする
  fn sort_students_by_age_ascending() {
    let taro = Student::new("Taro", "Yamada", 16);
    let hanako = Student::new("Hanako", "Yamada", 14);
    let kyoko = Student::new("Kyoko", "Ito", 15);
    let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

    // ソート対象のベクタを作成する
    let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

    // ソート後の期待値を作成する
    let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

    assert_eq!(
      // sort_by関数でソートする。第2引数はソート順を決めるクロージャ
      // 引数に2つのStudent構造体をとり、ageフィールドの値をcmpメソッドで
      // 比較することで大小を決定する
      sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
      Ok(())
    );

    // 結果を検証する
    assert_eq!(x, expected);
  }

  #[test]
  fn sort_students_by_name_ascending() {
    let taro = Student::new("Taro", "Yamada", 16);
    let hanako = Student::new("Hanako", "Yamada", 14);
    let kyoko = Student::new("Kyoko", "Ito", 15);
    let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

    let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
    let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

    assert_eq!(
      sort_by(
        &mut x,
        // まずlast_nameを比較する
        &|a, b| a
          .last_name
          .cmp(&b.last_name)
          // もしlast_nameが等しくない(LessまたはGreater)ならそれを返す
          // last_nameが等しい(Equal)ならfirst_nameを比較する
          .then_with(|| a.first_name.cmp(&b.first_name))
      ),
      Ok(())
    );
    assert_eq!(x, expected);
  }

  #[test]
  fn sort_u32_large() {
    {
      let mut x = new_u32_vec(65536);

      assert_eq!(sort(&mut x, &Ascending), Ok(()));
      assert!(is_sorted_ascending(&x));
    }
    {
      let mut x = new_u32_vec(65536);

      assert_eq!(sort(&mut x, &Descending), Ok(()));
      assert!(is_sorted_descending(&x));
    }
  }
}
