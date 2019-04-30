extern crate gnuplot;

use gnuplot::{Figure, Caption, Color};

use std::thread::spawn;
use std::thread::JoinHandle;
use std::vec::Vec;

fn main() {
    // calculates first layer of functions
    let gerade_z=spawn(|| fn_eigenvalues(1.0, 0.0));

    // find ungerade zustände
    let ungerade_z = spawn(||fn_eigenvalues(0.0, 1.0));

    let g_eigenvals = gerade_z.join().unwrap();
    let u_eigenvals = ungerade_z.join().unwrap();

    for e in g_eigenvals{
        println!("gerade eigenwerte  : {}", e);
    }
    for e in u_eigenvals{
        println!("ungerade eigenwerte: {}", e);
    }
}

fn fn_eigenvalues(f_0: f64, f_0_s: f64) -> Vec<f64>{
    let mut result =
        exermine_peak(1000, 0.0, 200.0, 1.0, 1000000, 0.0, f_0, f_0_s, 40.0).iter().map(|(e,(xs, _))|
            (*e, xs.len() as f64)).collect();
    plot_tuples(&result);
        let mut peaks: Vec<JoinHandle<f64>> = Vec::new();
        for(p_e_start, p_e_end) in find_peaks(&result) {
            peaks.push(spawn( move || {
                let mut e_start = p_e_start;
                let mut e_end = p_e_end;
                for _ in 0..5{
                    let f = exermine_peak(100, 0.0, 200.0, 1.0, 1000000, 1.0, 0.0, e_start, e_end).iter()
                        .map(|(e,(xs,_))| (*e, xs.len() as f64)).collect();
                    let peaks = find_peaks(&f);
                    if peaks.len() == 0 {
                        let peak = f.iter().fold((0.0, 0.0), |(_,a), (e, x)|{
                            if *x <= a{
                                (*e, a)
                            }else{
                                (*e, *x)
                            }
                        });
                        let (peak_e, _) = peak;
                        let mut start = peak_e - (e_end - e_start);
                        let mut end  = peak_e + (e_end - e_start);
                        if start < e_start {
                            e_start = start;
                        }
                        if end > e_end {
                            e_end = end;
                        }
                    } else {
                        e_start=peaks[0].0;
                        e_end = peaks[0].1;
                    }
                }
                0.5 * (e_start+ e_end)
            }));
        }

        let mut return_v = Vec::new();
        for peak in peaks{
            return_v.push(peak.join().unwrap());
        }
        return_v
}

fn find_peaks(input: &Vec<(f64, f64)>) -> Vec<(f64, f64)>{
    let mut result: Vec<(f64, f64)> = Vec::new();
    let mut iter = input.iter();

    loop {
        // rising edge
        #[warn(unused_assignments)]
        let mut fst_x = 0.0;
        #[warn(unused_assignments)]
        let mut fst_y = 0.0;
        match iter.next(){
            Some((sfst_x, sfst_y)) => {
                fst_x = *sfst_x;
                fst_y = *sfst_y;
            },
            None => {break},
        }
        loop {
            match iter.next() {
                Some((x, y)) => {
                    if fst_y < *y {
                            break;
                        }
                    fst_x = *x;
                    fst_y = *y;
                },
                None => break,
            }
        }

        match iter.next(){
            Some((end_peak, _)) => result.push((fst_x , *end_peak)),
            None => break
        }

        //falling edge
        match iter.next(){
            Some((sfst_x, sfst_y)) => {
                fst_x = *sfst_x;
                fst_y = *sfst_y;
            },
            None => {break},
        }
        loop {
            match iter.next() {
                Some((x, y)) => {
                    if fst_y > *y {
                            break;
                        }
                    fst_x = *x;
                    fst_y = *y;
                },
                None => break,
            }
        }
    }
    result
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
