# intheo

Intheo は純粋関数型プログラミング言語です。カリー・ハワード対応を利用して定理を証明することも可能です。

## Intheo の設計

Intheo の設計には 1 つの基本原理があります。

* 全てを明確に行う。

Intheo の設計では、本当に必要になるまで抽象化は行いません。

特に、オブジェクト指向プログラミングで行われるような「データをロジックで隠蔽する」という手法は避けます。理由は 3 つあります。

* データをロジックで包んだものが基本単位になってしまう。これは基本単位としては大きすぎる。
* データとロジックは様々に組み替えられうるが、データをロジックで包んだものが基本単位となると、これが阻害される。
* データをロジックで隠蔽するのは、データの内容が無制限な更新により制御できなくなり、データを参照する処理が間接的に干渉し合ってしまうのを防ぐためであった。しかし、これは不変性を導入することで解決できる。

## Intheo の実装

Intheo は 3 つの段階を経て実装される予定です。

1. Rust により書かれたインタプリンタ
2. Rust により書かれたコンパイラ
3. Intheo 自身により書かれたコンパイラ

## Intheo の理論

Intheo は、宇宙の階層を明示的に取り扱い、数量型でデータの複製と破棄をコントロールし、自分型で帰納型を表現する。

Intheo の型理論は、 `Γ ⊢ x : T` という形の型判断しか持たない。また、型判断 `Γ ⊢ x : T` は推論木と一対一対応する。すなわち、 `x` は証明の過程を正確に表現している。

## Intheo の簡約器

Intheo の簡約機は [symmetric interaction calculus](https://github.com/Hexirp/Symmetric-Interaction-Calculus) を使って書かれます。

### 構文

構文は次の通りです。

```
t, s = x                          -- variable
     | lambda x => t              -- abstraction
     | t s                        -- application
     | { t, s }                   -- copied pair
     | match t with { x, y } => s -- matching / copying
```

スコープは一つしかないことに注意してください。すなわち、 `match t with { x, y } => s` と書いた時に `t` の中に `x` が現れたり、 `t (lambda x => s)` と書いた時に `t` の中に `x` が現れたりしても問題ありません。

### 簡約

簡約のルールは次の通りです。

```
-- rule 0 (lambda application)

(lambda x => t) s
-----------------
t [ x / s ]

-- rule 1 (pair projection)

match { u, v } with { x, y } => t
---------------------------------
t [ x / u ] [ y / v ]

-- rule 2 (pair application)

{ u, v } t
------------------------------------- (`x` and `y` are fresh)
match t with { x, y } => { u x, v y }

-- rule 3 (lambda projection)

match lambda x => t with { p, q } => s
--------------------------------------------------------------------------------------- (`y` and `z` are fresh)
match t with { p, q } => s [ p / lambda y => p ] [ q / lambda z => q ] [ x / { y, z } ]
```
