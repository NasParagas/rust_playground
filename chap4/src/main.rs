use std::vec;


fn main() {
    // 所有権の移動
    let s = vec!["udon".to_string(), "soba".to_string(), "ramen".to_string()];
    let t = s.clone();
    let u = s.clone();
    drop(t);
    drop(u);

    let v = vec![1,2];
    if v[0] > v[1]
    {
        mymove(v);
    }
    else
    {
        mymove(v);
    }
    // mymove(v);  // コンパイルエラー

    let v = vec![1,2];
    let mut v2 = v.clone();
    v2[1] = 1;
    for vi in v
    {
        println!("{}", vi);
    }
    for vi in v2
    {
        println!("{}", vi);
    }

    let i = 1;
    let i2 = i;
    let i3 = i;

    let l = Label {number:3};
    print(l);
    println!("hello! {}", l.number);
}

#[derive(Clone, Copy)]
struct StringLabel { name: String}

#[derive(Clone, Copy)]
struct Label { number: i64}

fn print(l: Label) { println!("Hello {}", l.number)}

fn mymove(v: Vec<i32>)
{
   let v2 = v;
}

fn non()
{

}