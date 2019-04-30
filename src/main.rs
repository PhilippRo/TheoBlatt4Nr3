extern crate gnuplot;

use gnuplot::{Figure, Caption, Color};

use std::thread::spawn;
use std::thread::JoinHandle;
use std::vec::Vec;

fn main() {
    // calculates first layer of functions

    //let fs_fn = exermine_peak(100, 0.0, 200.0, 1.0, 1000000, 1.0, 0.0, 0.0, 40.0);
    //plot_tuples(&fs_fn.iter().map(|(e, (xs, _))| (*e, xs.len() as f64)).collect());

    // find ungerade zustände
    let mut result: Vec<(f64, f64)> = Vec::new();
    let mut peaks: Vec<JoinHandle<Vec<(f64,f64)>>> = Vec::new();
    let mut ev_found : usize = 0;
    let mut e: f64 = 0.0;
    let e_step = 0.5;
    while ev_found <= 9 {
        // rising edge
        let (xs, _) = numerov(0.0, 200.0, 1.0, 1000000, 0.0, 1.0,
            |x| e - x*x, |_| 0.0);
        let mut fst_len = xs.len() as f64;
        result.push(( e, fst_len));
        loop {
            e = e + e_step;
            let (xs, _) = numerov(0.0, 200.0, 1.0, 1000000, 0.0, 1.0,
                |x| e - x*x, |_| 0.0);
            let snd_len = xs.len() as f64;
            if snd_len < fst_len  {
                break;
            }
            fst_len = snd_len;
            result.push(( e, fst_len));
        }
        // exermine peak
        peaks.push(spawn( move ||
            exermine_peak(100, 0.0, 200.0, 1.0, 1000000, 0.0, 1.0, e - 2.0 * e_step, e).iter()
            .map(|(e,(xs,_))| (*e, xs.len() as f64)).collect()));

        println!("{}", e);
        ev_found = ev_found + 1;
        println!("{}",ev_found);
        //rising edge
        let (xs, _) = numerov(0.0, 200.0, 1.0, 1000000, 0.0, 1.0,
            |x| e - x*x, |_| 0.0);
        let mut fst_len = xs.len() as f64;
        result.push(( e, fst_len));
        loop {
            e = e + e_step;
            let (xs, _) = numerov(0.0, 200.0, 1.0, 1000000, 0.0, 1.0,
                |x| e - x*x, |_| 0.0);
            let snd_len = xs.len() as f64;
            if snd_len > fst_len  {
                break;
            }
            fst_len = snd_len;
            result.push(( e, fst_len));
        }
    }
    for peak in peaks{
        result.extend(peak.join().unwrap().iter());
    }
    result.sort_by(|(e, _), (e1, _)| e.partial_cmp(e1).unwrap());
    plot_tuples(&result);

}

fn plot_tuples(plt: &Vec<(f64, f64)>) {
    let xs = plt.iter().map(|(x, _)| *x).collect::<Vec<f64>>();
    let ys = plt.iter().map(|(_,y)| *y).collect::<Vec<f64>>();

    let mut f = Figure::new();
    f.axes2d().lines(xs.as_slice(), ys.as_slice(), &[Caption("A line"), Color("black")]);
    f.show();
}

fn exermine_peak(many: usize, start_x: f64, end_x: f64, threshold: f64, steps: usize, y: f64,
    y_s: f64, start_e: f64, end_e: f64) -> Vec<(f64, (Vec<f64>, Vec<f64>))>{

    (0..many).map(|e_m|{
        let my_e = (end_e - start_e) * e_m as f64 / many as f64 + start_e;
        (my_e , numerov(start_x, end_x, threshold, steps, y, y_s,
            |x| my_e - x*x, |_| 0.0))}).collect::<Vec<_>>()
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
