use std::thread::{available_parallelism, spawn};

const THRESHOLD: usize = 5;

/// Computes the given function `f` on each element of the input vector `input`
/// in parallel using multiple threads.
///
/// If the input is small enough (less than the `THRESHOLD` constant), the computation is
/// performed in the main thread instead of spawning new threads.
///
/// # Examples
///
/// ```
/// use simple_parallel_compute::compute;
/// let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let output = compute(input, |t| t * 2);
/// assert_eq!(output, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
/// ```
pub fn compute<T, R>(input: Vec<T>, f: fn(t: T) -> R) -> Vec<R>
where
    T: Send + Clone + 'static,
    R: Send + 'static,
{
    let input_size = input.len();

    // If the input is small enough, just compute it in the main thread
    if input_size < THRESHOLD {
        return input.into_iter().map(f).collect();
    }

    let threads_count = available_parallelism()
        .expect("cannot get parallelism")
        .get();

    // The chunk size is calculated that way because we want to ensure that each chunk has roughly the same number of
    // elements, and that all elements are distributed evenly among the threads.
    let chunk_size = (input_size + threads_count - 1) / threads_count;

    let mut thread_handles = Vec::with_capacity(threads_count);

    input.chunks(chunk_size).for_each(|chunk| {
        let chunk = chunk.to_vec();

        thread_handles.push(spawn(move || chunk.into_iter().map(f).collect::<Vec<_>>()));
    });

    thread_handles
        .into_iter()
        .flat_map(|handle| handle.join().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::compute;

    #[test]
    fn test_compute_static_empty_input() {
        let input: Vec<i32> = vec![];
        let result = compute(input, |x| x * 2);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_compute_static_single_input() {
        let input = vec![1];
        let result = compute(input, |x| x * 2);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_compute_static_small_input() {
        let input = vec![1, 2];
        let result = compute(input, |x| x * 2);
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_compute_static_medium_input() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = compute(input, |x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    }

    #[test]
    fn test_compute_static_large_input() {
        let input = vec![1; 1000];
        let result = compute(input, |x| x * 2);
        assert_eq!(result, vec![2; 1000]);
    }

    #[test]
    fn test_compute_static_complex_function() {
        fn factorial(n: i32) -> i32 {
            if n <= 1 {
                1
            } else {
                n * factorial(n - 1)
            }
        }

        let input = vec![1, 2, 3, 4, 5];
        let result = compute(input, factorial);
        assert_eq!(result, vec![1, 2, 6, 24, 120]);
    }

    #[test]
    fn test_compute_static_long_computation() {
        let input = vec![1, 2, 3, 4, 5];
        let result = compute(input, |x| {
            std::thread::sleep(std::time::Duration::from_secs(2));
            x * 2
        });
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }
}
