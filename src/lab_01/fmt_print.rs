// Reference: https://doc.rust-lang.org/rust-by-example/hello/print.html
fn main() {
  println!("Hello, Rust!");

  println!("Hello, {}!", "vidge");
  println!("Hello, {1}!, My age is {0}.", 30, "vidge");
  println!(
    "Hello, {name}!, My age is {age}.",
    age = 26,
    name = "Vidge Wong"
  );

  println!("Format binary, {:b}!", 16);
  println!("Format octal, {:o}!", 16);
  println!("Format hex, 0x{:x}!", 13);
  println!("Format hex, 0x{:X}!", 14u32);

  // format hex and pad extra zeros
  println!("Format hex, 0x{:08X}!", 14u32);

  println!("{number:>5}", number = 123);
  println!("{name:>5}", name = "vidge wong");

  println!("{number:0>5}", number = 1);
}
