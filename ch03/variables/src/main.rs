fn main() {
  example1();
  example2();
  example3();
}

fn example1() {
  let mut x = 5;
  println!("The value of x is {}", x);
  x = 6;
  println!("The value of x is {}", x);
}

fn example2() {
  let tup = (500, 6.4, true);
  let (_x, y, _z) = tup;
  println!("The value of y is {} - direct access {} {} {}", y, tup.0, tup.1, tup.2);
}

fn example3() {
  let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
  println!("The month is {}", months[11]);
}
