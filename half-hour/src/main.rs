// https://fasterthanli.me/articles/a-half-hour-to-learn-rust

// signed integers   : i8, i16, i32, i64, i128 (bits)
// unsigned integers : u8, u16, u32, u64, u128
// type annotation using `:`
// variable names are always in snake_case
fn let_keyword() -> i8 {
  let a_variable = 2; // no type annotation
  let b_variable : i8 = 4;
  return a_variable + b_variable;
}

// the compiler won't warn about them being unused
fn throw_away() {
  let _ = 12;
  let _x = 24;
}

// shadowing : declare a new variable with the same name as a previous variable
// the second `x` is shadowing or re-defining the first `x`
// the function returns the second `x`
fn shadow_binding() -> i16 {
  let x : i16 = 36;
  let x = x + 3;
  return x;
}

// a tuple is a collection of values of different types
fn tuples() -> char {
  let pair = (48, 'a');
  let _type_annotation : (i8, char) = (12, 'b');
  let _firt_element = pair.0;
  let second_element = pair.1;
  return second_element;
}

fn destructured_tuple() -> char {
  let (some_char, _some_int) = ('c', 96);
  let (_another_char, _another_int) : (char, i16) = ('d', 32); // type annotation
  return some_char;
}

fn return_tuple() -> (char, char) {
  let pair : (char, char) = ('L', 'R');
  return pair;
}

// a pair of brackets declares a block, which has its own scope
fn block() {
  let x = "out";
  {
    let x = "in";
    println!("{}", x);
  }
  println!("{}", x);
}

// equivalent to `let x = 3; return x;`
fn block_tail() -> i16 {
  let x = {
    let a = 1;
    let b = 2;
    a + b // tail : what the whole block will evaluate to
  };
  return x;
}

// we don't need parenthesis around `2 > 1`
fn conditionals_are_expressions() -> bool {
  if 2 > 1 {
    true
  } else {
    false
  }
}

fn main() {
  println!("hello world");
  println!("{}", let_keyword());
  throw_away();
  println!("{}", shadow_binding());
  println!("{}", tuples());
  println!("{}", destructured_tuple());

  let (left, _) = return_tuple(); // throw away `right`
  println!("{}", left);

  block();
  println!("{}", block_tail());
  println!("{}", conditionals_are_expressions());
}
