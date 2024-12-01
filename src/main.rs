use std::{
    io::{self, Write},
    process::Command,
};

fn main() {
    loop {
        // 画面にシェルの受け付けを表示　flushで即時に標準出力に表示
        print!("プロンプト> ");
        let _ = io::stdout().flush();

        // ユーザからのインプット（シェルのコマンド）を受ける
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // 空白や改行を取り除く
        let command = input.trim();

        // コマンド実行
        let mut child = Command::new(command).spawn().unwrap();

        // 子プロセスが終了するまで待つ
        let _ = child.wait();
    }
}
