extern crate gnuplot;

use gnuplot::{Figure, Caption, Color};

use std::vec::Vec;

fn main() {
    // calculates first layer of functions
    let many = 300;
    let e = 40.0;

    let fs_fn = (0..many).map(|e_m|{
        println!("{}", e_m);
        (e_m as f64 * e as f64 / many as f64 ,
        numerov(0.0, 200.0, 1.0, 1000000, 1.0, 0.0,
            |x| e_m as f64* e as f64 / many as f64 - x*x, |_| 0.0))}).collect::<Vec<_>>();

    let plt = fs_fn.iter().map(
        |(e, (xs, _))| (*e, xs.len() as f64)).collect::<Vec<(f64, f64)>>();
    let xs = plt.iter().map(|(x, _)| *x).collect::<Vec<f64>>();
    let ys = plt.iter().map(|(_,y)| *y).collect::<Vec<f64>>();

    let mut f = Figure::new();
    f.axes2d().lines(xs.as_slice(), ys.as_slice(), &[Caption("A line"), Color("black")]);
    f.show();
}


fn numerov<ST: FnMut(f64) -> f64, GT: FnMut(f64) -> f64>(x_0: f64, x_n: f64, thr: f64,
    n: usize, y_0: f64, y_1: f64, para_g: GT, para_s: ST) -> (Vec<f64>, Vec<f64>) {

    let mut s = para_s;
    let mut g = para_g;

    let h = (x_n - x_0) / n as f64;
    let h_q = h*h;
    let mut xs = (0..n).map(|ni| x_0 + (x_n - x_0) * ni as f64 / n as f64)
        .collect::<Vec<f64>>();
    let mut ys = vec![y_0, y_0 + y_1 * h];

    for i in 1..(n-1) {
        let y =  (2.0 * ys[i] * (1.0 - (5.0 * h_q * g(xs[i])) / 12.0) -
            ys[i-1]*(1.0 + (h_q * g(xs[i-1]))/12.0) +
            h_q / 12.0  * (s(xs[i+1]) * 10.0 *s(xs[i]) + s(xs[i-1])))
            / (1.0 + h_q * g(xs[i+1])/12.0);
        if thr <= y.abs() {
            break;
        }else{
            ys.push(y);
        }
    }

    xs.truncate(ys.len());
    (xs, ys)
}
