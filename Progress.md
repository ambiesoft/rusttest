https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html

* cargo.lockで再現可能なビルドが行える
最初に作った時の状態で他の環境でもビルドできる
'cargo update'でアップデートできる

* 'cargo doc --open'でプロジェクトが使っているクレートのドキュメントが開ける

* 可変な参照は１つしかもてない
* ダングリングポインタは絶対に存在しない
* 文字列リテラルはスライスである
```
let s = "Hello, world!";
```
ここでのsの型は、&strです。不変な参照です。

https://doc.rust-jp.rs/book-ja/ch05-00-structs.html
https://doc.rust-jp.rs/book-ja/ch06-00-enums.html
