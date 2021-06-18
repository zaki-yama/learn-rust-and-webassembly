## When to Call Functions from WAT

- `(export)`
- JS から Wasm の関数の呼び出しはオーバーヘッドがあるので、小さなタスクを実行するだけの関数であれば export すべきでないよ。JS でやった方がいい
- Wasm の関数は大量のデータを何回もループで処理するようなものに適している
- デバッグについては Chapter 10
- パフォーマンスチューニングの過程で、元々関数として切り出してたものをインライン化することもあるかもしれない
  - パフォーマンス・チューニングについて詳しくは Chapter 9 で

## Writing an `is_prime` Function

###

- `local.tee`: The local.tee command is like the local.set command in that it sets the value of the variable you pass to it to the value on top of the stack

### Performance Implications of External Function Calls

- "When you call a JavaScript function
  in WAT, you lose some cycles to overhead. This number isn’t extremely large, but if you execute an external JavaScript function in a loop that iterates 4,000,000 times, it can add up."
