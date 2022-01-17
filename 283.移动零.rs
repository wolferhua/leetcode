/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 *
 * https://leetcode-cn.com/problems/move-zeroes/description/
 *
 * algorithms
 * Easy (63.83%)
 * Likes:    1392
 * Dislikes: 0
 * Total Accepted:    594.4K
 * Total Submissions: 928.4K
 * Testcase Example:  '[0,1,0,3,12]'
 *
 * 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
 * 
 * 示例:
 * 
 * 输入: [0,1,0,3,12]
 * 输出: [1,3,12,0,0]
 * 
 * 说明:
 * 
 * 
 * 必须在原数组上操作，不能拷贝额外的数组。
 * 尽量减少操作次数。
 * 
 * 
 */

// @lc code=start
impl Solution {
    // 声明两个快慢指针，每一次循环快指针都向前进，如果快指针指向的元素不是0，则赋值到慢指针所在位置，同时慢指针向前移动一步，快指针指向位置变为0(相当于快慢指针指向元素交换)。
    // 需要注意如果快指针和慢指针处于同一位置(一般是起始)且不为0，则不要把快指针指向元素设置为0。 
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0; // 移动数据的位置记录
        for i in (0..nums.len()){ 
            if nums[i]!=0{ // 判断是否是零，如果不是零跳过
                nums[j] = nums[i]; // 将非 0 数据移动到前边去
                if i != j { // i!=j 说明不是当前位置，将当前位置设置为零
                    nums[i] = 0;
                }
                j+=1; // 移动后非 0 位置增长
            }
        }
    }
}
// @lc code=end

