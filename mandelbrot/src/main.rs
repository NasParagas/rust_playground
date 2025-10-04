/*
 * やろうとしていること
 *  マンデルブロ集合
 *      zを複素数とし、z = z*z + cの計算を無限に繰り返したとき、zが無限大にならないcの集合
 *  cがマンデルブロ集合に含まれる場合には黒、含まれない場合には白でプロット
 *  
 */

// use num::Complex;
// /*
//  * Complexの定義の<T>によって、Complex<f64>とするとフィールドreとimがともにf64となるComplexを生成することができる
//  */

// fn complex_square_add_loop(c: Complex<f64>) {
//     let mut z = Complex {re:0.0, im:0.0};
//     loop {
//         z = z * z + c;
//     }
// }

use num::Complex;
use std::{std::FromStr, str::FromStr};

/**
 * pub enum Option<T> {
    /// No value.
    #[lang = "None"]
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    /// Some value of type `T`.
    #[lang = "Some"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
 */

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);  // マンデルブロ集合に含まれている場合
        }
        z = z * z + c;
    }
    None  // 含まれていると言えない場合
}

/// 
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                
            }
        }
    }
}