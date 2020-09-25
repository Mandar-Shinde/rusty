#[derive(Debug)]
struct Person<'a>
{
    name: &'a str,
    age: u8
}

struct Point{
    x:i32, y:i32
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

fn draw(event:Canvas){
    match event{
        Canvas::Whiteboard=> println!("_Whiteboard"),
        Canvas::Dot(_,_)=> println!("_Dot"),
        Canvas::Rect=> println!("_Rect"),
    }
}

fn main() {
    println!("Hello, world!");

    // struct operation
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    let point_obj : Point=Point {x:1,y:1};
    let point_new = Point{x:3,..point_obj};
    println!("new point ({}, {})", point_new.x,point_new.y);

    let idot = Canvas::Dot(3,4);
    draw(idot);

}