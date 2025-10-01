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

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    
}