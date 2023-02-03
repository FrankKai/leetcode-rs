impl Solution {
  pub fn detect_capital_use(word: String) -> bool {
      /**
       * 将char转化为大小写：.to_ascii_uppercase(),.to_ascii_lowercase()
       * 遍历vec时获取index：vec.iter().enumerate()
       */
      let vec: Vec<char> = word.chars().collect();
      let allUppercase = vec.iter().all(|&x| x == x.to_ascii_uppercase());
      let allLowercase = vec.iter().all(|&x| x == x.to_ascii_lowercase());
      let mut firstUppercase = false;
      for (pos, &e) in vec.iter().enumerate() {
          if pos == 0 && e == e.to_ascii_uppercase() {
              firstUppercase = true;
          } else if e != e.to_ascii_lowercase() {
              firstUppercase = false
          }
      }
      return allUppercase || allLowercase || firstUppercase;
  }
}