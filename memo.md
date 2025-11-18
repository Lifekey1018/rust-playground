# rust学習用メモ

## 2025/11/18

### Hellow World

[チュートリアル](https://doc.rust-jp.rs/book-ja/ch01-02-hello-world.html)

```rust
fn main() { // 関数の定義
    println!("Hello, world!"); // 関数ではなくマクロ？を呼び出す
}
```

関数の定義はfnから始まる. mainは関数名で, ()には引数が入る
関数本体は{}で囲む
println!はマクロの呼び出し. マクロは!で識別される
マクロとはなにかはチュートリアルの19章で説明される

コンパイルと実行

```sh
$ rustc main.rs
$ ./main
> Hello, world!
```

rustcはRustのコンパイラ、main.rsはソースコードファイル名
./mainは生成された実行ファイル

main関数は特別な関数で、常に全ての実行可能なRustプログラムで走る最初のコードになる。

### Hellow, Cargo!

[チュートリアル](https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html)

CargoはRustのビルドシステム件パッケージマネージャ

cargo newコマンドで新しいプロジェクトを作成できる

```sh
$ cargo new hello_cargo
```

cargoはsrcディレクトリがあることを期待し、プロジェクトの最上位ディレクトリにはREADME.mdやCargo.tomlといった設定ファイル等が配置される。

Cargo.tomlはプロジェクトのメタデータや依存関係を記述するファイル

cargo buildコマンドでプロジェクトをビルドできる

```sh
$ cargo build
```

このコマンドによりCargoはsrc/main.rsを探し、コンパイルしてtarget/debugディレクトリに実行ファイルを生成する

cargo runコマンドでビルドと実行を同時に行える

```sh
$ cargo run
``` 

cargo checkコマンドでコンパイルのみを行い、実行ファイルは生成しない

```sh
$ cargo check
```
このコマンドはコードの正当性を確認するのに役立つ

リリースに向けたビルドはcargo build --releaseコマンドを使用する

```sh
$ cargo build --release
```

このコマンドは最適化された実行ファイルを生成し、target/releaseディレクトリに配置する
つまり、開発中は`cargo build`または`cargo run`を利用し、リリースする際には`cargo build --release`を利用する

### Guessing Game

[チュートリアル](https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html)

ミュータブルとイミュータブルの違い

Rustではデフォルトで変数はイミュータブル（不変）である。つまり、一度値を設定すると変更できない。

`&`参照
参照ってなんだ？
参照は変数の値へのポインタのようなもので、所有権を移動せずに値にアクセスできる方法を提供する。
所有権とは
Rustの所有権システムは、メモリ管理を安全かつ効率的に行うための中心的な概念である。各値は所有者と呼ばれる変数に関連付けられており、その所有者がスコープを抜けると値は自動的に解放される。
→まったくわからん

エラーハンドリング
Rustでは、エラー処理にResult型を使用する。Result型はOkとErrの2つのバリアントを持ち、成功と失敗を表現する
Result型の目的は、エラーに関わる情報を符号化すること
expectメソッドでエラー時のメッセージを指定できる
Result型でexpectメソッドを使用しない場合には、コンパイル時にケイクが発生し、エラーの可能性を明示的に処理することを強制される

useキーワード
useキーワードは、外部クレートやモジュールから関数、構造体、列挙型などをスコープに導入するために使用される。これにより、コード内でそれらを直接使用できるようになる。→要はimportやincludeのようなもの

列挙型(enum)
値の集合を定義するための型
matchと一緒に使われることが多い
では、matchとはなんだ？

match
上見識の一種で評価時に列挙型の値がどの列挙子であるかに基づいて異なるコードを実行できるようにするもの

マクロのプレースホルダー
{}: 引数を文字列に変換して埋め込む
{:?}: 引数をデバッグ形式で文字列に変換して埋め込む
{:#?}: 引数を整形されたデバッグ形式で文字列に変換して埋め込む
例えば次のように書ける
```rust
let x = 5;
let y = 10;
println!("x = {}, y = {}", x, y); // 出力: x = 5, y = 10
```
