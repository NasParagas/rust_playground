use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

// dyn=dynamic
// 静的ではなく動的に型を決定するが、少なくともErrorトレイトではある...みたいな感じ
type MyResult<T> = Result<T, Box<dyn Error>>;

// pub = privateでなくする
pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}
