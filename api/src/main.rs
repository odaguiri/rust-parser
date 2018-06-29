#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

// basic types have special behaviors, but custom vars needs to add copy so it can assign a new value
#[derive(Clone, Copy)]
struct PointCopy {
  x: i32,
  y: i32,
}

fn print_point(point: &Point) {
  println!("#print_point: point(x: {}, y: {})", point.x, point.y);
}

fn print_point_copy(point: PointCopy) {
  println!("#print_point: point(x: {}, y: {})", point.x, point.y);
}

fn inc_x(point: &mut PointCopy) {
  point.x += 1;
}


fn main() {
  // var type
  let name: &str = "world";
  println!("Hello {}!", name);

  // mutable var
  let mut age = 1;
  age += 1;
  println!("Age should be {} not 1!", age);

  // set struct
  let p1 = Point { x: 11, y: 22 };

  // set var with same index
  let p2 = &p1;

  // use fn
  print_point(&p1);

  // print ignoring debug
  println!("p1 = {:#?}", p1);
  println!("p2 = {:#?}", p2);

  // clone var
  let p3 = PointCopy { x: 32, y: 34 };
  let p4 = p3.clone();
  let mut p5 = PointCopy { x: 0, y: 0 };

  print_point_copy(p3);
  print_point_copy(p4);

  inc_x(&mut p5);
  print_point_copy(p5);
}
