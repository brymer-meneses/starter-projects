use text_io::read;
use rand::random;
use indicatif::ProgressBar;

fn main() {
    println!("Monte-Carlo Approximation of pi!");

    println!("How many iterations?");
    let iterations: u64 = read!();
    
    println!("Doing {} iterations to approximate pi", iterations);


    let mut total_inside_points = 0;
    let mut i = 0; 
    let bar = ProgressBar::new(iterations);
    bar.set_style(indicatif::ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {Approximating..}")
    .progress_chars("#987654321-"));

    while i < iterations {
        i += 1;
        bar.inc(1);

        let x = random::<f64>();
        let y = random::<f64>();
        let distance = (x.powf(2.0) + y.powf(2.0)).sqrt();
       
        if distance < 1.0 {
            total_inside_points += 1;
        }
    }

    bar.finish();
    let approx_pi = 4.0*(total_inside_points as f64/iterations as f64);

    println!("Pi Approximation: {:.10}", approx_pi);

}
