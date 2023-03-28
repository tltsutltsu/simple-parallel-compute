use simple_parallel_compute::compute;

fn main() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let output = compute(input, |t| t * 2);

    println!("{:?}", output);
}
