impl Solution {
  pub fn are_occurrences_equal(s: String) -> bool {
      /**
       * .chars()得到字符串的iterator
       * HashMap统计每个数字出现的个数
       * unwrap_or(0)在map中没有该键对应的value时设置默认值为零
       * .into_values() 获取到HashMap的values
       * .iter()获取到vector的iterator
       * .all()遍历判断每一项是否都满足条件
       */
      use std::collections::HashMap;
      let mut map = HashMap::new();
      for char in s.chars() {
          map.insert(char, map.get(&char).copied().unwrap_or(0) + 1);
      }
      let vec: Vec<i32> = map.into_values().collect();
      let result = vec.iter().all(|&x| x == vec[0]);
      return result;
  }
}