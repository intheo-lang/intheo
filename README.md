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

## Intheo の理論

Intheo の型理論は、 `Γ ⊢ x : A & x_Multiplicity` という形の型判断しか持たない。また、型判断は推論木と一対一対応する。

シークエント計算を元にしている。判断同値も値である。定義は変数と判断同値の組合せで表される。

Intheo の理論では、次の記述を可能にするつもりである。

* 適用 (application)
  * pure type system をベースとする。
* 抽象 (abstraction)
  * pure type system をベースとする。
* 変数 (variable)
  * pure type system をベースとする。
* 依存関数型 (dependent function type)
  * pure type system をベースとする。
* 型の型 (type of type)
  * pure type system をベースとする。
* 多相型 (polymorphic type)
  * pure type system をベースとする。
* 型関数 (type function)
  * pure type system をベースとする。
* 依存型 (dependent type)
  * pure type system をベースとする。
* 宇宙階層 (universe hoerarchy)
  * universe hierarchy を組み込む。
* 線形型 (linear type)
  * quantitative type theory を組み込む。
* 道型 (path type)
  * cubical type theory を組み込む。
* 区間型 (interval type)
  * cubical type theory を組み込む。
* 合成 (composition)
  * cubical type theory を組み込む。
* 単一型 (unit type)
  * 組み込む。
* 空型 (void type)
  * 組み込む。
* 直積型 (product type)
  * 組み込む。
* 直和型 (sum type)
  * 組み込む。
* 依存直積型 (dependent product type)
  * 組み込む。
* 依存直和型 (dependent sum type)
  * 組み込む。
* 関数型 (function type)
  * 依存関数型を使って実装する。
* ウ型 (W-type)
  * 組み込む。
* ム型 (M-type)
  * 組み込む。
* 帰納型 (inductive type)
  * ウ型を使って定義する。なお、 "[W-types: good news and bad news](https://mazzo.li/epilogue/index.html%3Fp=324.html)" に記載されているように、ウ型を使ってリスト型を定義することも可能である。なお、ウ型は関数外延性が必要なのが難点とされているが、 cubical type theory を組み込むので問題ない。なお、一時期は [Yatima 言語](https://github.com/yatima-inc/yatima)に倣って自分型を使うことも考えたが、[考察](https://github.com/Hexirp/blog)の結果で、自分型が帰納型の実装に使えるのは偶然的なものであるという結論に至ったので、ウ型を使うことにした。
* 余帰納型 (coinductive type)
  * ム型を使って定義する。
* 帰納帰納型 (inductive-inductive type)
  * "[inductive-inductive type in nLab](https://ncatlab.org/nlab/show/inductive-inductive+type)" によると "[Constructing Inductive-Inductive Types in Cubical Type Theory](https://link.springer.com/chapter/10.1007/978-3-030-17127-8_17)" (Jasper Hugunin) が homotopy type theory に親和的な形で帰納帰納型から帰納型への翻訳を定義している。これを利用する。
* 商型 (quotient type)
  * 組み込む。
* 高階帰納型 (higher inductive type)
  * 商型を使って定義する。
* 高階帰納帰納型 (higher inductive-inductive type)
  * 帰納帰納型と商型を使って定義できるか……？

ちなみに、 "W-type" を "ウ型" と訳したのは "W" を "ウ" と読んだだけのことである。 "W 型" と訳することも検討したが "W" と "type" がハイフンで繋がれているという雰囲気が出せていないと感じた。また、 "M-type" を "ム型" に訳したのは、同様に "M" を "ム" と読んだだけのことである。 "ウ" と "ム" が 180 度だけ回転させた形になっているのは偶然である。

### ソート

`Sort : Sort` である。

### 多重度の型

`Multiplicity : Sort` である。

### 多重度

`Multiplicity.zero : Multiplicity` である。

`Multiplicity.one : Multiplicity` である。

`Multiplicity.omega : Multiplicity` である。

`Multiplicity.plus : pi Multiplicity type -> pi Multiplicity type -> Multiplicity` を導出できる。

`Multiplicity.multiply : pi Multiplicity type -> pi Multiplicity type -> Multiplicity` を導出できる。

`Multiplicity` と `Multiplicity.zero` と `Multiplicity.plus` は可換モノイドを作らなければならない。

`Multiplicity` と `Multiplicity.one` と `Multiplicity.multiply` はモノイドを作らなければならない。

`Multiplicity.plus` と `Multiplicity.multiply` は分配法則を満たさなければならない。

`Multiplicity.zero` は `Multiplicity.multiply` において零元でなければならない。

`Multiplicity.plus x y = Multiplicity.zero` と `x = Multiplicity.zero ∧ y = Multiplicity.zero` は同値でなければならない。

`Multiplicity.multiply x y = Multiplicity.zero` と `x = Multiplicity.zero ∨ y = Multiplicity.zero` は同値でなければならない。

### レベルの型の型

`Type_Maximum : Sort`

### レベルの型

`Level : Type_Maximum` である。

### レベル

レベルは整列集合でなければならない。

### レベル述語の型

`Level_Predicate : Sort` である。

### 区間の型

`Interval : Sort`

### 区間

区間はブール代数でなければならない。

### 区間述語の型

`Interval_Predicate`

### 関数の型

`A : Type i` かつ `x : A ⊢ B : Type j` のとき、 `(pi x : A -> B) : Type j` である。

`i : Level ⊢ B : Type j` のとき、 `(pi i : Level -> B) : Type_Maximum` である。

`A : Type_Maximum` かつ `x : A ⊢ B : Type j` のとき、 `(pi x : A -> B) : Type j` である。

`i : Interval ⊢ B : Type j` のとき、 `(pi i : Interval -> B) : Type j` である。

`i : Multiplicity ⊢ B : Type j` のとき、 `(pi i : Multiplicity -> B) : Type j` である。

## Intheo のモジュール

`A.B.C.x` という識別子があるとします。

まず、そこにモジュール `A` がないかどうか検索します。次に `A` の中にモジュール `B` がないか検索します。次に `B` の中にモジュール `C` がないか検索します。次に `C` の中に値 `x` がないか検索します。それが `A.B.C.x` が指すものです。

この手順のいずれかで失敗したとき、一つ上のモジュールに上がって同じことを繰り返します。

それでも見つからない場合はエラーとなります。
