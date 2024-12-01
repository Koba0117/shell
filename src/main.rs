use std::{io, process::Command};

fn main() {
    // ユーザからのインプット（シェルのコマンド）を受ける
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // 空白や改行を取り除く
    let command = input.trim();

    // コマンド実行
    Command::new(command).spawn().unwrap();
}
