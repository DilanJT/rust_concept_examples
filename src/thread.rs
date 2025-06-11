// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::{ops::AddAssign, sync::{Arc, Mutex}, thread};

pub fn sum(v: Vec<i32>) -> i32 {
    let sum: Arc::<Mutex<i32>> = Arc::new(Mutex::new(0));

    let mid = v.len() /2;
    println!("Vector: {:?}", v);
    let (left, right) = v.split_at(mid);
    let left_owned: Vec<i32> = left.to_vec();
    println!("Left: {:?}", left_owned);

    let right_owned: Vec<i32> = right.to_vec();
    println!("Right: {:?}", right_owned);
    
    let sum_left_clone = sum.clone();
    let sum_right_clone = sum.clone();

    let left_sum_calc = thread::spawn(move || {
        if let Ok(mut sum_left) = sum_left_clone.lock() {
            *sum_left += left_owned.iter().sum::<i32>();
        } else {
            eprintln!("Failed to acquire left lock: mutex might be poisoned");
        }
    });

    let right_sum_calc = thread::spawn(move || {
        if let Ok(mut sum_right) = sum_right_clone.lock() {
            *sum_right += right_owned.iter().sum::<i32>();
        } else {
            eprintln!("Failed to acquire right lock: mutex might be poisoned");
        }
    });

    let _ = left_sum_calc.join();
    let _ = right_sum_calc.join();

    let final_sum = sum.lock().unwrap();
    *final_sum
}

fn main() {
    let numbers = vec![];
    let result = sum(numbers);
    println!("The sum is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
