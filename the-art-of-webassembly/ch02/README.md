# Chapter 2 WebAssembly Text Basics

この Chapter で紹介すること

- 2 comment styles in WebAssembly
- 伝統的な hello world アプリケーション
  - 文字列扱うのが難しいので、いきなり hello world はやらない
- import object を使って JS から Wasm にデータをインポートする方法
- named and unnamed global and local variables
- S 式と、wat2wasm コンパイラーが S 式をどうアンパックするか
- if/else , branch tables, loops, blocks

## Writing the Simplest Module

すべての WAT アプリケーションはモジュールでなければならない。よって最初は module シンタックスから始まる

```wat
(module
  ;; This is where the module code goes.
)
```

- 複数行コメントは `(; ... ;)`

## Hello World in WebAssembly

### Creating Our Wat Module

- linear memory

```wat
(module
  (import "env" "print_string" (func $print_string( param i32 )))
  (import "env" "buffer" (memory 1))
)
```

- `(memory 1)` は buffer が linear memory 1 ページになることを示している
  - page とは
    - linear memory に一度に割り当てることのできる最小のメモリの塊
    - Wasm では 1 ページ 64KB

以下はスキップ

- if/else Conditional Logic
- Loops and Blocks
- The loop Expression

## WAT Variables

### Unpacking S-Expressions

```wat
(i32.mul
  (i32.add
    (i32.const 3)
    (i32.const 2)
  )
  (i32.sub
    (i32.const 9)
    (i32.const 7)
  )
)
```

は

```wat
i32.const 3
i32.const 2
i32.add

i32.const 9
i32.const 7
i32.sub

i32.mul
```

と等価

## Loops and Blocks

### The `block` Statement

```wat
;; This code is for demonstration and not part of a larger app
(block $jump_to_end
  br $jump_to_end

  ;; code below the branch does not execute. br jumps to the end of the block
  nop
)
;; This is where the br statement jumps to
nop
```

- `br` で `block` を抜ける (the code can only jump to the end of a `block` if it’s inside that `block`)

## The loop Expression
