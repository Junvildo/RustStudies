fn main() {
    let p1: Point<i32> = Point { x: 6, y: 8 };
    let p2: Point<f64> = Point { x: 3.25, y: 8.63};
    println!("{:?}", p1);
    println!("{:?}", p2);

    let c1 = Color::Red("f88");
    let c2 = Color::Red(255);
    println!("{:?}", c1);
    println!("{:?}", c2);

    let p3:Point2<i32, f64> = Point2 { x: 34, y: 8.5 };
    println!("{:?}", p3);
}

#[derive(Debug)]
struct Point<T> {
    x:T,
    y:T
}

#[derive(Debug)]
enum Color<T> {
    Red(T),
    Blue(T),
    Green(T)
}


#[derive(Debug)]
struct Point2<T,V> {
    x:T,
    y:V
}
