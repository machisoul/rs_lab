// Reference: https://doc.rust-lang.org/rust-by-example/hello/print.html

#[derive(Debug)]
struct Structure(i32, u32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

fn main() {
  println!(
    "1. Hello, {name:?}!, My age is {age:?}, and my height is {0:?}\n",
    180,
    age = 26,
    name = "Vidge Wong"
  );

  println!("2. debug output for structure, {:?}\n", Structure(13, 14).1);

  let name = "Peter";
  let age = 27;
  let peter = Person { name, age };
  println!("3. {:#?}\n", peter);
}
