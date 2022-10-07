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

https://doc.rust-jp.rs/book-ja/ch06-03-if-let.html
https://doc.rust-jp.rs/book-ja/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#%E3%83%A2%E3%82%B8%E3%83%A5%E3%83%BC%E3%83%AB%E3%83%84%E3%83%AA%E3%83%BC%E3%81%AE%E8%A6%81%E7%B4%A0%E3%82%92%E7%A4%BA%E3%81%99%E3%81%9F%E3%82%81%E3%81%AE%E3%83%91%E3%82%B9