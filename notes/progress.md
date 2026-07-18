# Rust学習チェックリスト

## ✅ 習得済み

### 環境・ツール
- [x] cargo の基本（`run` / `check` / `fmt`、`--vcs none`）
- [x] コンパイラのエラーメッセージの読み方（`help:` 行を必ず読む）

### 基本文法
- [x] `let` — デフォルト不変、`mut`、シャドーイング、型注釈（`i32` / `u32`）
- [x] `if` — 括弧不要・波括弧必須・条件は`bool`のみ・**式として値を返す**
- [x] `for` と Range（`0..5` / `0..=5`）、`while` / `loop`
- [x] 式ベースの戻り値 — ブロック最終式（セミコロンなし）が値になる。`return`は早期リターン用
- [x] `println!` / `format!` — マクロ（`!`必須）、フォーマットのコンパイル時チェック、引数は自動借用
- [x] `match` — タプルパターン、網羅性チェック、パターンの順序、`_`ワイルドカード
- [x] 命名規約 — 関数/変数は`snake_case`、型は`UpperCamelCase`（コンパイラが警告する）

### 所有権・借用（Rustの核心）
- [x] ムーブセマンティクス — 代入・関数渡しはデフォルトでムーブ（E0382）
- [x] `Copy`トレイト — `i32`等はコピー、`String`等はムーブ
- [x] `.clone()` — 深いコピーは明示的に
- [x] 借用 `&T` — 複数可・読み取り専用。呼び出し側にも`&`を書く
- [x] 可変借用 `&mut T` — 排他的に1個だけ（E0502）
- [x] 借用の寿命は「最後の使用まで」（NLL）
- [x] 可変性の3点セット — `let mut` / `&mut 変数` / `引数: &mut T`（E0596 / E0308）
- [x] `String` vs `&str` — 引数は`&str`が慣用、スライス、deref coercion

### 構造体・メソッド
- [x] `struct`の定義・生成・フィールドアクセス、借用で受け取る関数
- [x] メソッド — `impl`ブロック、`&self` / `&mut self`、レシーバは括弧内に書かない（自動参照）
- [x] `fn new() -> Self` — コンストラクタは言語機能でなく慣例。`::`呼び出し、フィールド省略記法
- [x] `#[derive(Debug)]`と`{:?}` / `{:#?}` — アトリビュート全般、フォーマット指定子の構造（Python互換）
- [x] Clone と Copy の関係（概念のみ。structへのCopy deriveの実験は未実施）

### enum
- [x] `enum` — データ付きバリアント（直和型）、バリアントは型でなく値/コンストラクタ
- [x] enumへの`impl`、`match self`での場合分け
- [x] 数値型の暗黙変換なし — `f64`に整数リテラル不可、変換は`as`で明示

### Option
- [x] `Option<T>` — nullの代替。`Some`/`None`の作成とmatchでの取り出し、`Option<f64>`と`f64`は別型
- [x] `.chars().next()` — イテレータの`next()`は`Option`を返す（Pythonと違い例外でない）。`s[0]`不可の理由（UTF-8可変長）
- [x] `if let` / `unwrap_or` / `unwrap`・`expect`の使い分け

### Result とエラー処理
- [x] `Result<T, E>`の基本 — `Ok`/`Err`のmatch、`.parse()`は型明示が無難（ターボフィッシュ`::<>`、E0284）、例外機構はない
- [x] `?`演算子 — エラー伝播の1文字化。使う関数自身がResultを返す必要がある。連発できる
- [x] `panic!`との使い分け — 起こりうる失敗はResult、バグはpanic
- [x] エラー型は変換先が決める（`FromStr`の`ParseIntError`等）
- [x] 複数エラー型の概観 — `Box<dyn Error>` / 自作enum / anyhow（使い込みは未実施）

### コレクション
- [x] `Vec<T>` — `vec!`マクロ、push、`[]`（範囲外は実行時panic）vs `.get()`（Option）、`&v`で借用ループ、`for &n in`パターン、要素参照とpushのE0502（イテレータ無効化の静的防止）、Rangeとの違い（IntoIterator）
- [x] `HashMap<K, V>` — `use`（includeと違い名前の別名）、insert/get、entry APIとEntry enum、`*`による書き込みデリファレンス、列挙順は毎回変わる（BTreeMapならキー順）

### イテレータ
- [x] `.iter()` / `into_iter()`（ムーブ）、`filter` / `map` / `sum` / `collect`、遅延評価
- [x] クロージャ（`|x| ...`）と参照パターン（`|&&x|`）
- [x] `sort_by`と`cmp`、タプルの`.0`/`.1`
- [x] パターン＝値の形を写した鋳型（match / if let / let / for / 引数の5箇所で使える）

### ファイルI/OとCLI基礎
- [x] `std::env::args()` — 引数はイテレータ、`nth(1)`でOption、`cargo run --`の意味
- [x] `std::fs::read_to_string` — Result<String, io::Error>、`?`との組み合わせ
- [x] `main() -> Result<(), E>` — unit型`()`、`Ok(())`、Errで終了コード1
- [x] **マイルストーン: 単語頻度ランキングCLI完成**（sample.txt → ランキング表示）

### トレイト
- [x] トレイト基礎 — `trait`定義（シグネチャは`;`終わり）、`impl Trait for Type`（E0053: 契約と実装の一致）、`&impl Trait`引数、`<T: Trait>`との関係
- [x] interfaceとの違い — 後付け実装・静的ディスパッチ・言語の土台（演算子/for文もトレイト）
- [x] 継承がない理由と代替 — トレイト（多相）/コンポジション（再利用）/enum（閉じた階層）
- [x] デフォルトメソッド — 実装付きメソッド、`self.area()`など契約内メソッドを呼べる
- [x] `Box<T>`（unique_ptr相当）と`Vec<Box<dyn Trait>>`（動的ディスパッチ、C++仮想関数相当）
- [x] スーパートレイト `trait Shape: Debug`（紹介のみ）

### モジュールと外部クレート
- [x] `mod name;`によるファイル分割、`use`、デフォルト非公開と`pub`（E0603 / E0451: フィールドは個別にpub）
- [x] クレート/cargo/crates.ioの関係（海運メタファー）、TOML
- [x] `cargo add` — 依存追加、推移的依存とCargo.lock、フィーチャーフラグ（概観）、`rand`で実践

### ファイルシステムとツール
- [x] `fs::read_dir` — 二重のResult（ディレクトリを開く/列挙中の各エントリ）、`?`の2段構え、`let entry = entry?;`のシャドーイングイディオム
- [x] `DirEntry` — `.path()`（PathBuf）/ `.file_name()` / `.metadata()`、`Metadata::modified()`で`SystemTime`
- [x] `PathBuf`/`OsString`は`Display`未実装 — OSの文字列はUTF-8保証がないため。表示は`.display()`か`{:?}`
- [x] letチェーン — `if let ... && let ...`（edition 2024）、clippyの`collapsible_if`
- [x] `cargo clippy` — リンタ（clang-tidy相当）、`cargo fmt --check`との使い分け、コミット前チェックの習慣
- [x] **マイルストーン: ウォッチャー段階1（単発スキャン）完成**（Issue #2、git運用は`notes/git.md`）

## 🔄 いま取り組み中

- [ ] **CLIツール制作（最終目標）: ファイル変更ウォッチャー**（Issue #2）
  - [x] 段階1: 単発スキャン（read_dir、metadata、SystemTime）
  - [ ] 段階2: 差分検知 — `HashMap<PathBuf, SystemTime>`の比較、`#[test]`入門 ← 次回ここから
  - [ ] 段階3: ポーリングループ化（loop + thread::sleep）
  - [ ] 段階4: clapによる引数処理
- [ ] ライフタイム注釈 — `'a`。参照を返す関数で必要になる
- [ ] テスト — `#[test]`と`cargo test`
- [ ] （必要に応じて）ジェネリクス深掘り、Rc/RefCell、スレッド

## 📚 参考

- [The Rust Programming Language（日本語版）](https://doc.rust-jp.rs/book-ja/)
- `rustc --explain E0382` — エラーコードの詳細解説をターミナルで読める
