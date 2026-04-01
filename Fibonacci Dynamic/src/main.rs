fn fibonacci_dynamic(n: u64) -> u64 {
    let mut fib_nums: Vec<u64> = Vec::with_capacity(n as usize + 1); 
    fib_nums.push(0);
    fib_nums.push(1); 
    for i in 2..=n {
        let next_fib = fib_nums[i as usize - 1] + fib_nums[i as usize - 2];
        fib_nums.push(next_fib);
    }
    fib_nums[n as usize]
} 

fn main() {
    let n: u64 = 10; 
    let result = fibonacci_dynamic(n); 
    println!("The {}th Fibonacci number is: {}", n, result);
}
