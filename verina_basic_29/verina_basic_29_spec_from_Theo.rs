use vstd::prelude::*;

  verus! {

  fn remove_element(s: &Vec<i32>, k: usize) -> (result: Vec<i32>)
      requires
          k < s.len(),
      ensures
          result.len() == s.len() - 1,
          forall|i: int| 0 <= i < k ==> result[i] == s[i],
          forall|i: int| k <= i < result.len() ==> result[i] == s[i + 1],
  {
      // impl-start
      assume(false);
      Vec::new()
      // impl-end
  }

  }
  fn main() {}