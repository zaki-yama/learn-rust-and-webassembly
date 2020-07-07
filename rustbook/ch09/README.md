第9章 パーサを作る
===============

9-2-1 トークン
- 言語の文法のうち、終端記号と呼ばれるものがトークン(?)

### 9-3 構文解析

```
EXPR = EXPR3 ;
EXPR3 = EXPR3, ("+" | "-"), EXPR2 | EXPR2 ;
EXPR2 = EXPR2, ("*" | "/"), EXPR1 | EXPR1 ;
EXPR1 = ("+" | "-"), ATOM | ATOM ;
ATOM = UNUMBER | "(", EXPR3, ")" ;
UNUMBER = DIGIT, {DIGIT} ;
DIGIT = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
```

- LL(1)法と呼ばれる手法を再帰下降パーサとして実装する
- 抽象構文木
- `FromStr` を実装すると `some_str.parse::<Ast>` のようにparseメソッドが呼べる
