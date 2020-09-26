use std::convert::From;
use std::fmt;

#[derive(Debug)]
struct Person<'a>
{
    name: &'a str,
    age: u8
}

struct Point{
    x:i32, y:i32
} 

impl From<i32> for Point{
    fn from(item: i32) -> Self {
        Point { x: item ,y: 0}
    }
}

impl fmt::Display for Point{
    fn fmt(&self ,f: &mut fmt::Formatter)->fmt::Result{
        write!(f," point is {}",self.x)
    }
}
// struct of struct
struct Rect{
    up:Point,down:Point
}

enum Canvas{
    Whiteboard,
    Rect,
    Dot(i32,i32)
}

static _IAMSTATIC: &str ="static";
const _IAMCONST:i32=1; 
type _UpperCamelCase =u64; // Aliasing

fn draw(event:Canvas){
    match event{
        Canvas::Whiteboard=> println!("enum _Whiteboard"),
        Canvas::Dot(_,_)=> println!("enum _Dot"),
        Canvas::Rect=> println!("enum _Rect"),
    }
}

fn main() {
    println!("Hello, world!");

    // struct operation
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // stuct
    let point_obj : Point=Point {x:1,y:1};
    let point_new = Point{x:3,..point_obj};
    println!("struct new point ({}, {})", point_new.x,point_new.y);

    // enum
    let idot = Canvas::Dot(3,4);
    draw(idot);

    // from into
    let num1 = Point::from(30);
    println!("from into point ({}, {})", num1.x,num1.y);

    // tostring
    println!("tostring {}", num1.to_string());

    //ref + guard
    let value = 1;
    match value {
        ref r if r>=&0i32  =>println!("guard  0<{:?}", r),
        _=>println!("0"),
    }

   // binding value
   match value {
    n @ 1 =>println!("binding  0<{:?}", n),
        _=>println!("0"),
    }

    //closure
    fn  clofun   (i: i32) -> i32 { i + 1 }
    let clovar =|i| i+1;
    println!("var: {} ,function: {} ", clovar(value),clofun(value));

    //capturing
}