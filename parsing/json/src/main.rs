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

fn parsejson()
{
    let parsed = json::parse(r#"
    {
        "name": "some name",
        "age": 15,
        "canJSON": true
    }
"#).unwrap();

println!("Printed:\n {}",parsed);

println!("Printed Pretty:\n {:#}",parsed)
}

fn main() {
    printjson();
    parsejson();
}
