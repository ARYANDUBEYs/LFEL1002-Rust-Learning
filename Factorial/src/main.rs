fn factorial (n:u64)->u64{
    if n==0 || n==1{
        1
    }
    else {
        n*factorial(n-1)
    }
}
fn main(){
    let n:u64 = 5;
    println!("The factorial of {} is {}", n, factorial(n));
}
