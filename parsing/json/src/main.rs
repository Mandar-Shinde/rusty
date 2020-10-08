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

println!("Printed Pretty:\n {:#}",parsed);

println!("Printed val:\n {:#}",parsed["age"]);
    if(parsed["canJSON"].as_bool().unwrap())
    {
        println!(" must be true");
    }else{
        println!(" must be false");
    }
}

fn main() {
    printjson();
    parsejson();
}
