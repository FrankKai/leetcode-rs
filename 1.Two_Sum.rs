impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 步骤
        // 从一组数字中，找出相加的和为target的索引
        // 双指针法：如何定义2个指针，相加为target时，返回索引
        let mut i = 0;
        let mut j = 1;
        let mut vec: Vec<i32> = Vec::new();
        // 如何遍历动态数组,通过while遍历
        while j < nums.len() {
            if (nums[i] + nums[j] == target) {
                vec.push(i as i32);
                vec.push(j as i32);
                break;
            }
            if (j == nums.len() - 1) {
                i += 1;
                j = i + 1;
            } else {
                j += 1
            }
        }
        return vec;
    }
}
