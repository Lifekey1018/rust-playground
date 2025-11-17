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

