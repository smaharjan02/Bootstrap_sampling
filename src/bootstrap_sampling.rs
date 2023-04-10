use rand::seq:: SliceRandom;
use std::{time::Instant};


//generating bootstrapping sample using simple random sampling with replacement
pub fn random_sample_with_replacement(sample: &Vec<i64>, size: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut resampled = Vec::with_capacity(size);

    for _ in 0..size {
        let item = sample.choose(&mut rng).unwrap().clone();
        resampled.push(item);
    }

    resampled
}

//generating bootstrapping sample groundtuth using simple random sampling with replacement 
pub fn bootstrap_sums(data: &Vec<i64>, num_resamples: usize, sample_fraction: f64) -> (Vec<i64>, f64) {
    let mut bootstrap_sums = Vec::with_capacity(num_resamples);

    // Start the timer
    let start_time = Instant::now();

    for _ in 0..num_resamples {
        let resampled_data = random_sample_with_replacement(&data, data.len());
        let sum: i64 = resampled_data.iter().sum();
        bootstrap_sums.push((sum as f64 / sample_fraction) as i64);
    }

    // Calculate the elapsed time
    let elapsed_time = start_time.elapsed().as_secs_f64();

    (bootstrap_sums, elapsed_time)
}


//calculating mean of bootstrapping ground truth sample
pub fn calculate_mean(bootstrap_sums: &[i64]) -> f64 {
    let sum: i64 = bootstrap_sums.iter().sum();
    sum as f64 / bootstrap_sums.len() as f64
}

//calculating standard deviation of bootstrapping ground truth sample
pub fn calculate_std_error(bootstrap_sums: &[i64], mean: f64) -> f64 {
    let count = bootstrap_sums.len() as f64;
    let variance: f64 = bootstrap_sums
        .iter()
        .map(|&value| {
            let diff = value as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / count-1.0;

    variance.sqrt()
}

//calculating confidence interval of bootstrapping ground truth sample
pub fn calculate_confidence_interval(std_dev: f64, z_score: f64) -> (f64, f64) {
    
    let lower_bound = z_score * std_dev;
    let upper_bound = z_score * std_dev;

    (lower_bound, upper_bound)
}

