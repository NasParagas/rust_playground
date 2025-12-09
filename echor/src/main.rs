use clap::{App, Arg};

fn main() -> () {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("NasParagas <naspara4693@gmail.com>")
        .about("Rust echor")
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
    // println!("{:#?}", std::env::args())

    println!("{:#?}", matches);
}
