// methods are functions attached to objects
struct Point {
  x: f64,
  y: f64,
}

// Implementation block; All `Point` methods go in here
impl Point {
  // This is a static method
  // static method don't need to be called by an instance
  // These methods are generally used as constructors
  fn origin() -> Point {
    Point {x: 0.0, y: 0.0}
  }

  // Another static method taking two arguments
  fn new(x: f64, y: f64) -> Point {
    Point {x, y}
  }
}

struct Rectangle {
  p1: Point,
  p2: Point,
}

impl Rectangle {
  // This is an isntance method
  // `&self` is sugar for `self: &Self` where `Self` is the type of the
  // caller object. In this case `Self = Rectangle`
  fn area(&self) -> f64 {
    let Point {x: x1, y: y1} = self.p1;
    let Point {x: x2, y: y2} = self.p2;

    ((x1 - x2) * (y1 - y2)).abs()
  }

  fn perimeter(&self) -> f64 {
    let Point {x: x1, y: y1} = self.p1;
    let Point {x: x2, y: y2} = self.p2;

    2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
  }

  // This method requires the caller object to be mutable
  // `&mut self` desugars to `self: &mut self`
  fn translate(&mut self, x: f64, y: f64) {
    self.p1.x += x;
    self.p1.y += y;
    
    self.p2.x += x;
    self.p2.y += y;
  }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
  // This method "consumes" the resources of the caller object
  // `self` desugars to `self: Self`
  fn destroy(self) {
    // Destructure self
    let Pair(first, second) = self;
    
    println!("Destroying pair ({}, {})", first, second);
  }
}

fn main() {
  let rectangle = Rectangle {
    p1: Point::origin(),
    p2: Point::new(3.0, 4.0),
  };

  // Instance method are called using dot operator
  // Note that the first argument `&self` is implicitly passed, i.e
  // `rectangle.perimeter()` === Rectangle::perimeter(&rectangle)`
  println!("Rectangle perimeter: {}", rectangle.perimeter());
  println!("Rectangle area: {}", rectangle.area());

  let mut square = Rectangle {
    p1: Point::origin(),
    p2: Point::new(1.0, 1.0),
  };

  // Error! `rectangle` is immutable, but this method requires a mutable object
  // rectangle.translate(1.0, 0.0)

  // Mutable objects can call mutable methods
  square.translate(1.0, 1.0);

  let pair = Pair(Box::new(1), Box::new(2));

  pair.destroy();

  // Error! previous `destroy` call consumed the pair hence the line below will throw error
  // pair.destroy();

}