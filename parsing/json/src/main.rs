use json::*;

fn printjson()
{
    let data = object!{
        name: "name tag",
        age: 30,
        canJSON: true
    };
    println!("Create:\n {:#}",data);
}

fn main() {
    printjson();
}
