/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 *
 * https://leetcode-cn.com/problems/squares-of-a-sorted-array/description/
 *
 * algorithms
 * Easy (71.28%)
 * Likes:    406
 * Dislikes: 0
 * Total Accepted:    231.1K
 * Total Submissions: 330.4K
 * Testcase Example:  '[-4,-1,0,3,10]'
 *
 * 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
 * 
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [-4,-1,0,3,10]
 * 输出：[0,1,9,16,100]
 * 解释：平方后，数组变为 [16,1,0,9,100]
 * 排序后，数组变为 [0,1,9,16,100]
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [-7,-3,2,3,11]
 * 输出：[4,9,9,49,121]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 10^4
 * -10^4 
 * nums 已按 非递减顺序 排序
 * 
 * 
 * 
 * 
 * 进阶：
 * 
 * 
 * 请你设计时间复杂度为 O(n) 的算法解决本问题
 * 
 * 
 */

// @lc code=start
impl Solution {
    // 双指针解法： https://leetcode-cn.com/problems/squares-of-a-sorted-array/solution/dai-ma-sui-xiang-lu-shu-zu-ti-mu-zong-ji-1rtz/
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();// 获取数组长度
        let (mut i,mut j,mut k) = (0,n-1,n-1);// 设置标识

        // 向量（Vector）是一个存放多值的单数据结构，该结构将相同类型的值线性的存放在内存中。
        // 向量是线性表，在 Rust 中的表示是 Vec<T>。
        // 向量的使用方式类似于列表（List），我们可以通过这种方式创建指定类型的向量：
        // https://www.runoob.com/rust/rust-collection-string.html
        let mut ans =vec![0;n];

        // 开始循环
        while i<=j {
            let numi = nums[i] * nums[i];
            let numj = nums[j] * nums[j];

            if numi < numj{
                ans[k] = numj;
                j-=1;
            }else{
                ans[k] = numi;
                i+=1;
            }
            k-=1;
        }


        // return ans;
        ans
    }
}
// @lc code=end

