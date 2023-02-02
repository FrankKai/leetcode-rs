impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
      /**
       * .to_string() 将数字转为string
       * .chars() 将string切片为iterator
       * .collect() 将iterator转为集合
       */
      let str = x.to_string();
      let str_vec: Vec<char> = str.chars().collect();
      let mut i = 0;
      let mut j = str_vec.len() - 1;
      while i < j {
          if str_vec[i] != str_vec[j] {
              return false;
          }
          i += 1;
          j -= 1;
      }
      return true;
  }
}