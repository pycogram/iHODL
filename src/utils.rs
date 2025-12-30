// src/utils.rs
use rand::seq::SliceRandom;

/// Shuffles a vector in-place using a thread-local RNG
pub fn shuffle_vec<T>(vec: &mut Vec<T>) {
    let mut rng = rand::thread_rng();
    vec.shuffle(&mut rng);
}

/// Selects `count` random elements from a slice without modifying the original
pub fn select_random<T: Clone>(vec: &[T], count: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let mut selected = vec.to_vec();
    selected.shuffle(&mut rng);
    selected.truncate(count);
    selected
}
