# Rust 学習の参照資料

## 知識

- [Rust Book：所有権](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
  公式の入門書。`String` のムーブ、関数への引数渡し、戻り値による所有権の移動を確認するときに使う。最初は「Ownership and Functions」と「Return Values and Scope」を読む。
- [Rust Book：参照と借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
  所有権を渡さずに値へアクセスする仕組みと、可変借用の制約を確認するときに使う。`&mut self` と、最初の先頭参照課題を考える際の参照先。
- [標準ライブラリ：VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
  出発点の実装が使うキューの公式仕様。投入・取り出しの順序や、空の場合の戻り値を確かめるときに使う。

## 実践知・コミュニティ

今回の焦点は、手元のコードとコンパイラで確かめられる所有権の移動。実務での設計判断を相談する段階で、目的に合うコミュニティを調べる。参加するかどうかの希望は未確認。
