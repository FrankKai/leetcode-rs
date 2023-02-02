impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        /**
         * .clone() 将nums变为mutable
         */
        let mut clonedNums = nums.clone();
        clonedNums.sort();
        while (clonedNums.len() > 0) {
            let first = clonedNums.pop();
            let second = clonedNums.pop();
            if (first != second) {
                return false;
            }
        }
        return true;
    }
}
