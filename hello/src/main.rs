// fn main() {
//     println!("Hello, world!");
// }

/*
 * mut...変数の値をプログラムの中で変更可能にする
 * let...ローカル変数の定義
 * !...関数呼び出しではなくマクロ呼び出しであることを示す
 * assert!...引数が真でない場合にメッセージを出してパニックさせる
 * returnは関数の途中で明示的に早期returnさせるときに用い、普段はセミコロンなしの式で終わらせることで返り値とする
 */
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// /*
//  * cargo testを実行するとコンパイルされて実行される
//  */
// #[test]
// fn test_gcd() {
//     assert_eq!(gcd(14, 15), 1);
// }

/*
 * FromStr...トレイト(>型が実装できるメソッドの集合)
 *  例えばu64::FromStrを使うためにはこの標準ライブラリのトレイトをスコープに取り込んでおく必要がある
 */
use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers: Vec<u64> = Vec::new();
    for arg in env::args().skip(1) { /* argsの一つ目はそのプログラムの名前になるのでskip */
        /*
         * (Rust analyzerで見るとわかりやすいが)from_strの返り値はResult型
         *  失敗する可能性のある関数は全てResult型を返す
         * Result型はOk(v), Err(e)の二つの形を取る
         *  Ok値には成功時の結果(転送されたバイト数やファイルなど)が、Err値には何がうまくいかなかったのかを示すエラーコードが収められる
         * Rustでは全てのエラーはResultかパニックで処理される(例外機構を持たない)
         */
        /* Resultのexpectメソッドでは、結果がOkであれば値vを返し、Errであればeの説明をするメッセージを出力してプログラムを即時終了する */
        numbers.push(u64::from_str(&arg)
               .expect("error parsing argument"));
    }

    /* eprintln!...標準エラー出力 */
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    /* &演算子...ここでは、変数numbersが所有しているvectorの二番目以降の要素をへの参照を借用している
     *  *で参照を解決している
     */
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    /* println!の記法？は(https://doc.rust-lang.org/std/fmt/index.html)を参照 */
    println!("The greatest common divisor of {:?} is {}", numbers, d);

    /* Rustではmainが何も返さなかったときにプログラムが成功と見做される(例えばC++は0を返す)
     * 逆に、expectなどで終了するときにはエラーを表す0以外のステータスコードを返す(例えばパニックで終了する場合は101)
     */
}