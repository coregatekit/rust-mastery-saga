// super is used to refer to the parent module

use super::mul::mul;

pub fn power(a: i32, b: i32) -> i32 {
    let mut result = 1;
    for _ in 0..b {
      result = mul(result, a);
    }
    result
}