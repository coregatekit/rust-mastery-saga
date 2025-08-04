// crate is refers to the root of the project, use for access to other modules

use crate::calculator::add::add;

pub fn mul(a: i32, b: i32) -> i32 {
  let mut result = 0;
  for _ in 0..b {
    result = add(a, b)
  }
  result  
}