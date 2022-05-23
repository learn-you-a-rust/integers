fn main() {
    let v = vec![2, 3, 5, 7 , 11, 13, 17, 19, 23, 29];
    let sum: usize = v.iter().sum();
    println!("the total sum is: {}", sum);
    
    let veclen: usize = v.len();
    let mean = sum / veclen;
    println!("the mean of vector v is: {}", mean);
}
