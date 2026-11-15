//! 同期・メモリ内の単一キューを使った学習用ブローカー。

use std::collections::VecDeque;

/// メッセージを投入順に保持するブローカー。
#[derive(Default)]
pub struct Broker {
    messages: VecDeque<String>,
}

impl Broker {
    /// 空のブローカーを作る。
    pub fn new() -> Self {
        Self::default()
    }

    /// 本文の所有権を受け取り、キューの末尾に入れる。
    pub fn publish(&mut self, message: String) {
        self.messages.push_back(message);
    }

    /// 先頭の本文を取り出して所有権を返す。空なら `None` を返す。
    pub fn receive(&mut self) -> Option<String> {
        self.messages.pop_front()
    }
}
