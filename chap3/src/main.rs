
fn main() 
{
    // 配列
    let array1 = [1,2,3,4,5];
    let array2 = ["monnnitiha"; 10];
    for elem in array2
    {
        println!("{}", elem);
    }

    // ベクタ
    // 初期化
    let mut primes = vec![2,3,5,7];
    let mut vec1 = vec![0; 3];
    let mut pal = Vec::new();

    // collectメソッドを使用した初期化
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages
    {
        println!("{}: {}",
            l, 
            if l.len() % 2 == 0 { "functional" }
            else { "imperative" }
        );
    }

    // メソッド使用例
    // push
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    assert_eq!(vec1, [0,0,0]);

    // pop
    pal.push("step");
    pal.push("on");
    assert_eq!(pal, ["step","on"]);
    pal.pop();
    assert_eq!(pal, ["step"]);

    // capasity
    let mut v = Vec::with_capacity(1);
    assert_eq!(v.capacity(), 1);
    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    println!("{}", v.capacity());
    v.push(1);
    v.push(2);
    v.push(2);
    println!("{}", v.capacity());

    // insert
    use quanta;
    vec1.insert(1, 1);
    assert_eq!(vec1, [0,1,0,0]);
    // let clock = quanta::Clock::new();
    // let mut vec2 = vec![0;1_000_000_000];
    // let start = quanta::Instant::now();
    // vec2.insert(0, 0);
    // let stop = quanta::Instant::now();
    // println!("{:?}", stop.duration_since(start));  // 0.25s程度
    // let start = quanta::Instant::now();
    // vec2.insert(900_000_000, 0);
    // let stop = quanta::Instant::now();
    // println!("{:?}", stop.duration_since(start));  // 0.015s程度

    // スライス
    let v = vec![0.1, 0.2, 0.3];
    let a =     [0.1, 0.2, 0.3];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;
    fn print_elem(n: &[f64]) -> ()
    {
        for elem in n 
        {
            println!("{}", elem);
        }
    }
    // どっちでも使える
    print_elem(sv);
    print_elem(sa);
    print_elem(&sv[0..1]);
    print_elem(&sa[1..]);

    // 文字列型
    let noodles = "noodle".to_string();
    let oodles = &noodles[1..];
    let poodles = "œ_œ";
    println!("{}", poodles.len());

    // 型エイリアス
    type Color = Vec<u8>;

}
