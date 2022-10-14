impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 合并数组，此时类型是Vec<i32>
        let mut concatenated: Vec<i32> = [nums1, nums2].concat();
        concatenated.sort();
        let isOdd: bool = concatenated.len() % 2 == 1;
        let idx = concatenated.len() / 2;
        let mut result: f64 = 0.0;
        if (isOdd) {
            // 通过into()将i32转为f64
            result = concatenated[idx] as f64;
        } else {
            result = ((concatenated[idx] as f64 + concatenated[idx - 1] as f64) / 2.0);
        }
        return result;
    }
}
