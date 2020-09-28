use std::mem;

struct Point{
    x:i32,
    y:i32,
}
fn obj() -> Point {
    Point { x: 0, y: 0 }
}

fn main() {
    //box
    let p: Box<Point> = Box::new(Point{x:1,y:1});
    let o: Box<Point> = Box::new(obj());

    //vec
    let mut v1 = vec![1i32,2,3];
    let v2: Vec<i32> = (0..15).collect();

    v1.push(3);
    v1.pop();
    v1.len();

    //array muly 2 
    for x in v1.iter_mut() {
        *x *= 2;
    }


    //str
    let s1: &'static str ="te st 1 2";
    let mut s2: Vec<char> = s1.split_whitespace().rev().char.collect();

    
}
