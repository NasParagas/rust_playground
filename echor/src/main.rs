fn hello() -> () {
    println!("hello");
}

fn main() -> () {
    let mut a: Vec<()> = vec![];
    a.push(());
    hello();
    let b :() = ();
}
