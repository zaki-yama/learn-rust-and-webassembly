pub mod first;
pub mod second;
pub mod third;

pub mod utils;

// SortOrderを列挙型として定義する
pub enum SortOrder {
  // SortOrder には2つのバリアントがある
  Ascending,
  Descending,
}
