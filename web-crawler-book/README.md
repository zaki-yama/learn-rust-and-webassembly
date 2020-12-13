[Rustで簡易ウェブクローラを作る by zenn](https://zenn.dev/shotaro_tsuji/books/32df27b4cc54df4fa7a5)
===

## 02 ウェブページを取得する

```rust
response.text()
```

の後 `response` にアクセスすると

```
borrow of moved value: `response`
value borrowed here after moverustcE0382
```

=> Chapter 03 で触れられてた

> レスポンスのボディを取り出す`text`メソッドが`Response`オブジェクトを消費してしまうことに注意します

## 03 リンクを抽出する

`filter_map`: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map

- 引数にわたすクロージャは `Option<T>` を返す関数
- クロージャの結果が `Some<T>` のもののみ return し、 `None` だと return しない
- 自動的に unwrap する

```rust
let a = ["1", "lol", "3", "NaN", "5"];

let mut iter = a.iter().filter_map(|s| s.parse().ok());

assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(3));
assert_eq!(iter.next(), Some(5));
assert_eq!(iter.next(), None);
```

ref. https://doc.rust-lang.org/1.41.0/std/iter/trait.Iterator.html#method.filter_map

> Why `filter_map` and not just `filter` and `map`? The key is in this part:
> 
>     If the closure returns `Some(element)`, then that element is returned.
> 
> In other words, it removes the `Option<T>` layer automatically. If your mapping is already returning an `Option<T>` and you want to skip over `None`s, then `filter_map` is much, much nicer to use.


## 07 幅優先探索を実装する

> 頂点を取り出すグラフは`AdjacentNodes`トレイトを実装したオブジェクトへの参照を持つことにします。なので型パラメータに参照のライフタイムが必要になります。

?

