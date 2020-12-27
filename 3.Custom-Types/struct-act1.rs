#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let length: f32 = rect.top_left.y - rect.bottom_right.y;
    let breadth: f32= rect.top_left.x - rect.bottom_right.x;

    (length * breadth).abs()
}

fn main() {
    let top_left = Point {x: 0.4, y: 10.3};
    let bottom_right = Point {x: 5.2, y: 0.4};

    let rectangle = Rectangle {
        top_left,
        bottom_right,
    };

    println!("{:?}", rectangle);
    println!("Area: {}", rect_area(rectangle));
}