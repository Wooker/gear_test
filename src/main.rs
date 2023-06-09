#![allow(unused)]

use std::{marker::Sync, ops::Add, panic, process, string::ToString, sync::Arc, thread};

const THRESHOLD: usize = 2;

fn split_work<T, R>(data: Vec<T>, func: fn(&T) -> R) -> Vec<R>
where
    T: Send + Sync + 'static,
    R: Clone + Send + 'static,
{
    let mut result: Vec<Vec<R>> = vec![];

    // Set the hook to handle panics
    panic::set_hook(Box::new(move |panic_info| {
        println!("Thread panicked. No processing.");
        process::exit(0);
    }));

    // Do not split if the input data size is less
    // than the THRESHOLD value
    if data.len() <= THRESHOLD {
        return data.iter().map(func).collect();
    }

    // Count the number of chunks
    let mut chunks_count = data.len() / THRESHOLD;
    if data.len() % THRESHOLD > 0 {
        chunks_count += 1;
    }

    // Create Arc to pass the data into threads
    let arc_data: Arc<Vec<T>> = Arc::new(data);

    // Compute the portions in separate threads
    for i in 0..chunks_count {
        let d = Arc::clone(&arc_data);
        result.push(
            thread::spawn(move || {
                let id = thread::current().id();
                println!("Split. {:?}, {}", id, i);

                // Select the chunk for current thread and apply the function
                let chunks: &[T] = d.chunks(THRESHOLD).nth(i).unwrap();
                chunks.iter().map(|s| func(s)).collect()
            })
            .join()
            .unwrap(),
        );
    }

    // Unite the computed data and return
    // R is Clone to allow the result to be concatenated
    result.concat()
}

// Returns a string from a type that
// implements the *to_string* method
fn to_string<T>(input: &T) -> String
where
    T: ToString,
{
    input.to_string()
}

// Returns a doubled value
fn add_itself<T>(input: &T) -> T
where
    T: Add + Add<Output = T> + Copy,
{
    *input + *input
}

// Panics
fn will_panic<T>(input: &T) -> T
where
    T: Add + Add<Output = T> + Copy,
{
    panic!();
    *input + *input
}

fn main() {
    let data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    dbg!(&data);
    let result = split_work(data, add_itself as fn(&i32) -> i32);
    dbg!(&result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Vectors of i32 and f32 are tested with each of the
    // functions (to_string, add_itself).
    // The last test checks the panic behavior.

    #[test]
    fn test_ints_to_string() {
        let data: Vec<i32> = vec![1, 2, 3, 5, 6];
        let data_length = data.len();
        let result = split_work(data, to_string as fn(&i32) -> String);

        assert_eq!(result, vec!["1", "2", "3", "5", "6"]);
        assert_eq!(result.len(), data_length);
    }
    #[test]
    fn test_floats_to_string() {
        let data: Vec<f32> = vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let data_length = data.len();
        let result = split_work(data, to_string as fn(&f32) -> String);

        assert_eq!(result, vec!["1.1", "2.2", "3.3", "4.4", "5.5", "6.6"]);
        assert_eq!(result.len(), data_length);
    }

    #[test]
    fn test_ints_add_itself() {
        let data: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let data_length = data.len();
        let result = split_work(data, add_itself as fn(&i32) -> i32);

        assert_eq!(result, vec![2, 4, 6, 8, 10, 12]);
        assert_eq!(result.len(), data_length);
    }

    #[test]
    fn test_floats_add_itself() {
        let data: Vec<f32> = vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let data_length = data.len();
        let result = split_work(data, add_itself as fn(&f32) -> f32);

        assert_eq!(result, vec![2.2, 4.4, 6.6, 8.8, 11.0, 13.2]);
        assert_eq!(result.len(), data_length);
    }

    #[test]
    fn more_data() {
        let data: Vec<i32> = vec![
            17, 100, 28, 6, 59, 36, 80, 78, 89, 97, 78, 40, 59, 26, 88, 41, 39, 100, 77, 87, 90,
            99, 56, 50, 49, 4, 98, 64, 3, 20, 43, 61, 49, 43, 22, 66, 43, 74, 52, 16, 1, 50, 89,
            87, 47, 89, 94, 72, 52, 38, 28, 48, 67, 79, 12, 11, 33, 10, 3, 32, 22, 49, 26, 37, 78,
            57, 89, 73, 17, 20, 59, 40, 38, 16, 6, 80, 49, 54, 20, 3, 32, 72, 66, 15, 94, 31, 29,
            83, 42, 30, 26, 98, 41, 30, 68, 39, 90, 63, 81, 17,
        ];
        let data_length = data.len();
        let data_doubled: Vec<i32> = data.clone().into_iter().map(|i| i + i).collect();
        let result = split_work(data, add_itself as fn(&i32) -> i32);

        assert_eq!(result, data_doubled);
        assert_eq!(result.len(), data_length);
    }

    #[test]
    fn panics() {
        let data: Vec<f32> = vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let data_length = data.len();
        let result = split_work(data, will_panic as fn(&f32) -> f32);

        assert_eq!(result, vec![2.2, 4.4, 6.6, 8.8, 11.0, 13.2]);
        assert_eq!(result.len(), data_length);
    }
}
