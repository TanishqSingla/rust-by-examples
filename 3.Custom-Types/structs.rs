#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified as top, left, bottom and right
    // corners in space
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};
    
    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point {x: 10.3, y: 0.4};

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point {x: 5.2, ..point};

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructuring a point using a `let` binding
    let Point{x: top_edge, y:left_edge} = point;

    let _rectangle = Rectangle {
        top_left: Point {x: top_edge, y: left_edge},
        bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    
    println!("pair contains {:?} and {:?}", integer, decimal);

}