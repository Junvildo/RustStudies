fn main() {
    let i = 5;
    let j = i;
    println!("{}",j);
    println!("{}",i);

    let v = vec![1, 2, 3, 4, 5];
    //let w = v;
    //println!("{:?}", w);
    //println!("{:?}", v);
    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vector use in foo");
        v
    };

    let v = foo(v);
    println!("{:?}", v);
}
