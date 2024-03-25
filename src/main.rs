/*!
 use rand::prelude::*;
use rayon::prelude::*;

fn main() {
    // Parameters
    let num_trials: usize = 1000000;
    let seq_length: usize = 1000;
    let prob_success: f64 = 0.5;

    // Calculate maximum run lengths in parallel
    let max_run_lengths: Vec<usize> = (0..num_trials)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let sequence: Vec<usize> = (0..seq_length)
                .map(|_| if rng.gen::<f64>() < prob_success { 1 } else { 0 })
                .collect();

            let max_length_in_trial = sequence
                .iter()
                .fold((0, 0), |(mut current_run_length, mut max_length), &sample| {
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
*/

use rand::prelude::*;
use rand_distr::Binomial;
use rayon::prelude::*;

fn main() {
    // Parameters
    let num_trials: usize = 1_000_000;
    let seq_length: usize = 10_000;
    let prob_success: f64 = 0.5;

    // Create a binomial distribution
    let binomial = Binomial::new(1, prob_success).unwrap();

    // Calculate maximum run lengths in parallel
    let max_run_lengths: Vec<usize> = (0..num_trials)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();

            // Generate the binary sequence for this trial
            let sequence: Vec<usize> = (0..seq_length)
                .map(|_| binomial.sample(&mut rng) as usize)
                .collect();

            // Calculate the maximum run length in the sequence
            let mut max_length = 0;
            let mut current_length = 0;
            for &bit in &sequence {
                if bit == 1 {
                    current_length += 1;
                    if current_length > max_length {
                        max_length = current_length;
                    }
                } else {
                    current_length = 0;
                }
            }
            max_length
        })
        .collect();

    // Calculate the mean of the maximum run lengths
    let mean_max_length: f64 = max_run_lengths.iter().sum::<usize>() as f64 / max_run_lengths.len() as f64;
    println!("Mean of maximum run lengths: {}", mean_max_length);
}
