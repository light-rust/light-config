#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_point() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_yaml::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_yaml::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
