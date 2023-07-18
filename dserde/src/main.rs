use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    id: String,
    other: usize,
}

fn main() {
    let record = MyStruct {
        id: String::from("HelloKitty"),
        other: 1000,
    };

    let record_str = r#"
        {
            "_id": "HelloKitty",
            other": 1000
        }
    "#;
    println!(
        "serialize: {:?}",
        serde_json::to_string(&record)
    );

    let p: Result<MyStruct, _> = serde_json::from_str(&record_str);

    if let Err(e) = p {
        println!("{}", e.to_string());
    }

    // println!("deserialize: {p:?}");
}
