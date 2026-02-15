use clap::{Arg, Command};
fn main() -> () {
    let matches = Command::new("echo-rs")
        .version("0.1.0")
        .about("echo in rust")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n') // -n をフラグとして使用可能にする
                .help("Do not print newline")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches(); // 引数を解析
    //
    // 変数matchesの中身↓
    // println!("{:#?}", matches);
    // let text: Vec<String> = ... とすることで型を明示して宣言する
    let text: Vec<String> = matches
        .get_many::<String>("text");
        // .expect("TEXT is required")
        // .map(|s| s.to_owned())
        // .collect();
    let omit_newline = matches.get_flag("omit_newline");
    let mut ending = "\n";
    if omit_newline {
        ending = "";
    }
    print!("{}{}", text.join(" "), ending);
    // 上記はまとめて
    // print!("{}{}", text.join(" "), if omit_newline { "\n" } else { "" });
    // ともかける(Rustではifは値(ここではbool)を返す)
}
