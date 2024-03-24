
/*!use rand::prelude::*;
use rand_distr::Binomial;
use rayon::prelude::*;

fn main() {
    // Parameters
    let num_trials: i32 = 100000;
    let prob_success: f64 = 0.5;

    // Calculate maximum run lengths in parallel
    let max_run_lengths: Vec<usize> = (0..num_trials)
        .into_par_iter()
        .map(|_| {
            let mut rng: ThreadRng = rand::thread_rng();
            let binomial: Binomial = Binomial::new(num_trials as u64, prob_success).unwrap();
            let num_successes: usize = binomial.sample(&mut rng) as usize;

            // Calculate the maximum run length
            let mut current_run_length: usize = 0;
            let mut max_length_in_trial: usize = 0;

            for _ in 0..num_successes {
                let sample: i32 = if rng.gen::<f64>() < prob_success {
                    1
                } else {
                    0
                };

                if sample == 1 {
                    current_run_length += 1;
                } else {
                    if current_run_length > max_length_in_trial {
                        max_length_in_trial = current_run_length;
                    }
                    current_run_length = 0;
                }
            }

            if current_run_length > max_length_in_trial {
                max_length_in_trial = current_run_length;
            }

            max_length_in_trial
        })
        .collect();

    // Calculate the mean of the maximum run lengths
    let mean_max_length: f64 = max_run_lengths.iter().sum::<usize>() as f64 / max_run_lengths.len() as f64;
    println!("Mean of maximum run lengths: {}", mean_max_length);
}*/

use rand::prelude::*;
use rand_distr::Binomial;
use rayon::prelude::*;

fn main() {
    // Parameters
    let num_trials: usize = 100000;
    let prob_success: f64 = 0.5;

    // Calculate maximum run lengths in parallel
    let max_run_lengths: Vec<usize> = (0..num_trials)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let binomial = Binomial::new(num_trials as u64, prob_success).unwrap();
            let num_successes = binomial.sample(&mut rng) as usize;

            let max_length_in_trial = (0..num_successes)
                .map(|_| {
                    let sample = if rng.gen::<f64>() < prob_success { 1 } else { 0 };
                    sample
                })
                .fold((0, 0), |(mut current_run_length, mut max_length), sample| {
                    if sample == 1 {
                        current_run_length += 1;
                        if current_run_length > max_length {
                            max_length = current_run_length;
                        }
                    } else {
                        current_run_length = 0;
                    }
                    (current_run_length, max_length)
                })
                .1;

            max_length_in_trial
        })
        .collect();

    // Calculate the mean of the maximum run lengths
    let mean_max_length: f64 = max_run_lengths.iter().sum::<usize>() as f64 / max_run_lengths.len() as f64;
    println!("Mean of maximum run lengths: {}", mean_max_length);
}
