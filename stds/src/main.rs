// growable Strings like: "hello world"
// growable vectors: [1, 2, 3]
// optional types: Option<i32>
// error handling types: Result<i32, i32>
// heap allocated pointers: Box<i32>

use std::mem;
use std::collections::HashMap;

struct Point{
    x:i32,
    y:i32,
}

fn obj() -> Point {
    Point { x: 0, y: 0 }
}

fn good(i:i32) -> Option<i32> {
    if(i<70)
    {
        None
    }else
    {
        Some(i)
    }
}
fn good1(i:i32) -> Result<i32,String> {
    if(i<70)
    {
        Err("bad marks".to_string())
    }else
    {
        Ok(i)
    }
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
    //let mut s2: Vec<char> = s1.split_whitespace().rev().collect();

    //options
    match good(95){
        None=> println!("bad marks"),
        Some(v) => println!("good mark {}",v)
    }

   // Result
    match good1(5){
        Ok(o)=> println!("good mark {}",o),
        Err(e)=> println!("<{}>",e),
    }

    //? chaining - passing result to upper layer

    //Hashset
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Takes a reference and returns Option<&V>
     contacts.get(&"Daniel");
     contacts.insert("Daniel", "164-6743");
     contacts.remove(&"Ashley"); 

     //Reference couting
     // ARC -Auto reference counting
     
     



}
