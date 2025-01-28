// Stolen from https://github.com/TheAlgorithms/Rust

#[cfg(test)]
use std::cmp::Eq;

#[cfg(test)]
use std::cmp::PartialOrd;

#[cfg(test)]
use std::hash::Hash;

#[cfg(test)]
pub fn have_same_elements<T: PartialOrd + Eq + Hash>(a: &[T], b: &[T]) -> bool {
    use std::collections::HashSet;

    match a.len() == b.len() {
        true => {
            // This is O(n^2) but performs better on smaller data sizes
            //b.iter().all(|item| a.contains(item))

            // This is O(n), performs well on larger data sizes
            let set_a: HashSet<&T> = a.iter().collect();
            let set_b: HashSet<&T> = b.iter().collect();
            set_a == set_b
        }
        false => false,
    }
}

#[cfg(test)]
pub fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
pub fn is_descending_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] >= w[1])
}

#[cfg(test)]
pub fn generate_random_vec(n: u32, range_l: i32, range_r: i32) -> Vec<i32> {
    use rand::Rng;

    let mut arr = Vec::<i32>::with_capacity(n as usize);
    let mut rng = rand::rng();
    let mut count = n;

    while count > 0 {
        arr.push(rng.random_range(range_l..range_r + 1));
        count -= 1;
    }

    arr
}

#[cfg(test)]
pub fn generate_nearly_ordered_vec(n: u32, swap_times: u32) -> Vec<i32> {
    use rand::Rng;

    let mut arr: Vec<i32> = (0..n as i32).collect();
    let mut rng = rand::rng();

    let mut count = swap_times;

    while count > 0 {
        arr.swap(rng.random_range(0..n as usize), rng.random_range(0..n as usize));
        count -= 1;
    }

    arr
}

#[cfg(test)]
pub fn generate_ordered_vec(n: u32) -> Vec<i32> {
    generate_nearly_ordered_vec(n, 0)
}

#[cfg(test)]
pub fn generate_reverse_ordered_vec(n: u32) -> Vec<i32> {
    let mut arr = generate_ordered_vec(n);
    arr.reverse();
    arr
}

#[cfg(test)]
pub fn generate_repeated_elements_vec(n: u32, unique_elements: u8) -> Vec<i32> {
    use rand::Rng;

    let mut rng = rand::rng();
    let v = rng.random_range(0..n as i32);
    generate_random_vec(n, v, v + unique_elements as i32)
}

#[cfg(test)]
pub fn log_timed<F: FnOnce()>(test_name: &str, f: F) {
    use std::time::Instant;

    let before = Instant::now();
    f();
    println!("Elapsed time of {:?} is {:?}", test_name, before.elapsed());
}
