use std::time::{Duration, Instant};
use std::hint::black_box;
use quanta::Clock;

fn main() 
{
    let n: u32 = 1_000_000;
    let warmup: usize = 1_000;

    // 準備運動
    for _ in 0..warmup 
    {
        black_box(Instant::now());
    }
    let clock = Clock::new();
    for _ in 0..warmup 
    {
        black_box(clock.now());
    }

    // blackbox()でコンパイラに食われないようにする
    // ループ自体の実行時間計測
    let loop_start = clock.now();
    let dummy: u64 = 0;
    for _ in 0..n 
    {
        black_box(dummy);
    }
    let loop_total = loop_start.elapsed();

// std::time::Instant::now() の実行時間計測
    let t_start = Instant::now();
    for _ in 0..n 
    {
        black_box(Instant::now());
    }
    let t_total = t_start.elapsed();

    // quanta::Clock::now() の実行時間計測
    let q_start = clock.now(); // 合計時間計測は std::Instant でもOK
    for _ in 0..n 
    {
        black_box(clock.now());
    }
    let q_total = q_start.elapsed();

    // ループ自体の実行時間を引く
    let t_diff = t_total.saturating_sub(loop_total);
    let q_diff = q_total.saturating_sub(loop_total);

    let t_avg_ns = as_ns(t_diff) as f64 / n as f64;
    let q_avg_ns = as_ns(q_diff) as f64 / n as f64;

    println!("Loop total:               {:?}", loop_total);
    println!("std::Instant::now total:  {:?}", t_total);
    println!("quanta::Clock::now total: {:?}", q_total);
    println!("std - Loop  (avg ns):     {:.3} ns/call", t_avg_ns);
    println!("quanta - Loop  (avg ns):  {:.3} ns/call", q_avg_ns);
}

// Duration→ns
fn as_ns(d: Duration) -> u64 
{
    let ns = d.as_secs() as u128 * 1_000_000_000u128 + d.subsec_nanos() as u128;
    ns.min(u128::from(u64::MAX)) as u64
}
