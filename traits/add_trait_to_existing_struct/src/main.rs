trait Sumable<T> {
    fn sum(&self) -> T;
}

impl Sumable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum+=*i;
        }
        sum
    }
}


fn main() {
    let a = vec![1, 2, 3, 4, 5];
    println!("sum = {}", a.sum());
    //let b = vec![1.0, 2.0, 3.0];
    //println!("sum float = {}",b.sum());
}
