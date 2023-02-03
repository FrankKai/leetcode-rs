impl Solution {
  pub fn thousand_separator(n: i32) -> String {
      /**
       * 创建空字符串：String::new()
       * None检查：[Option].is_none()
       * 从Option中释放出值(Option<char> -> char)：.unwrap()
       * String中插入Char：.insert(idx, char)
       * Vec头部插入(类似js中array的unshift)：.rotate_right(1)
       */
      let str = n.to_string();
      let mut vec: Vec<char> = str.chars().collect();
      let mut stack: Vec<String> = Vec::new();
      let mut sub_str = String::new();
      while vec.len()> 0 {
          let pop = vec.pop();
          if !pop.is_none() {
              sub_str.insert(0, pop.unwrap());
          }
          if sub_str.len() == 3 {
              stack.push(sub_str);
              stack.rotate_right(1);
              sub_str = String::new();
          }
      }
      if sub_str.len() > 0 {
          stack.push(sub_str);
          stack.rotate_right(1);
      }
      let result = stack.join(".");
      return result;
  }
}