use clap::{App, Arg};

fn main() -> () {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("example <example.ex.co.ex>")
        .about("echo in rust")
        .arg(
            Arg::with_name("text") // ?
                .value_name("TEXT") // helpとかで出てきそう
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n") // -n で使用可能にする
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches(); // 引数を解析
    //
    // 変数matchesの中身↓
    // println!("{:#?}", matches);

    let text = matches.values_of_lossy("text").unwrap();
    // let text: Vec<String> = ... とすることで型を明示して宣言することもできる

    let omit_newline = matches.is_present("omit_newline");

    let mut ending = "\n";
    if omit_newline {
        ending = "";
    }
    print!("{}{}", text.join(" "), ending);
    // 上記はまとめて
    // print!("{}{}", text.join(" "), if omit_newline { "\n" } else { "" });
    // ともかける(Rustではifは値を返す。elseなしだとユニット型を返す)
}
