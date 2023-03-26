#![allow(unused)]

use std::{
    string::ToString,
    ops::Add,
    thread
};

const THRESHOLD: usize = 2;

fn split_work<T, R>(data: Vec<T>, func: fn(T) -> R) -> Vec<R> 
where
    // generics are Clone for the sake of simplicity,
    // since the input data type is Vec<T>
    T: Clone + Send + 'static, 
    R: Clone + Send + 'static {
    let mut result: Vec<Vec<R>> = vec![];

    // Do not split if the input data size is less
    // than the THRESHOLD value
    if data.len() <= THRESHOLD {
        return data.into_iter().map(func).collect();
    }

    // Otherwise, split the data 
    let chunks: Vec<Vec<T>> = data
        .chunks(THRESHOLD)
        .map(|s| s.into())
        .collect();

    // Compute the portions in separate threads
    for chunk in chunks.into_iter() {
        result.push(
            thread::spawn(move || {
                println!("Split");
                chunk.into_iter().map(func).collect::<Vec<R>>()
            })
            .join()
            .unwrap()
        );
    }

    // Unite the computed data and return
    result.concat()
}

// Returns a string from a type that
// implements the *to_string* method
fn to_string<T>(input: T) -> String
where
    T: ToString {
    input.to_string()
}

// Returns a doubled value
fn add_itself<T>(input: T) -> T
where
    T: Add + Add<Output = T> + Copy {
    input + input
}

fn main() {
    let data: Vec<i32> = vec![1, 2, 3, 4, 5];
    dbg!(&data);
    let result = split_work(data, to_string as fn(i32) -> String);
    dbg!(&result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Vectors of i32 and f32 are tested with each of the
    // functions (to_string, add_itself)

    #[test]
    fn test_ints_to_string() {
        let data: Vec<i32> = vec![1, 2, 3,  5, 6];
        let data_length = data.len();
        let result = split_work(data, to_string as fn(i32) -> String);

        assert_eq!(result, vec!["1", "2", "3", "5", "6"]);
        assert_eq!(result.len(), data_length);
    }
    #[test]
    fn test_floats_to_string() {
        let data: Vec<f32> = vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let data_length = data.len();
        let result = split_work(data, to_string as fn(f32) -> String);

        assert_eq!(result, vec!["1.1", "2.2", "3.3", "4.4", "5.5", "6.6"]);
        assert_eq!(result.len(), data_length);
    }

    #[test]
    fn test_ints_add_itself() {
        let data: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let data_length = data.len();
        let result = split_work(data, add_itself as fn(i32) -> i32);

        assert_eq!(result, vec![2, 4, 6, 8, 10, 12]);
        assert_eq!(result.len(), data_length);
    }

    #[test]
    fn test_floats_add_itself() {
        let data: Vec<f32> = vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let data_length = data.len();
        let result = split_work(data, add_itself as fn(f32) -> f32);

        assert_eq!(result, vec![2.2, 4.4, 6.6, 8.8, 11.0, 13.2]);
        assert_eq!(result.len(), data_length);
    }
}