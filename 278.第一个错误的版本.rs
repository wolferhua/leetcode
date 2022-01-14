/*
 * @lc app=leetcode.cn id=278 lang=rust
 *
 * [278] 第一个错误的版本
 *
 * https://leetcode-cn.com/problems/first-bad-version/description/
 *
 * algorithms
 * Easy (45.40%)
 * Likes:    410
 * Dislikes: 0
 * Total Accepted:    175.9K
 * Total Submissions: 387.4K
 * Testcase Example:  '5\n4'
 *
 * 
 * 你是产品经理，目前正在带领一个团队开发新的产品。不幸的是，你的产品的最新版本没有通过质量检测。由于每个版本都是基于之前的版本开发的，所以错误的版本之后的所有版本都是错的。
 * 
 * 假设你有 n 个版本 [1, 2, ..., n]，你想找出导致之后所有版本出错的第一个错误的版本。
 * 
 * 你可以通过调用 bool isBadVersion(version) 接口来判断版本号 version
 * 是否在单元测试中出错。实现一个函数来查找第一个错误的版本。你应该尽量减少对调用 API 的次数。
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 5, bad = 4
 * 输出：4
 * 解释：
 * 调用 isBadVersion(3) -> false 
 * 调用 isBadVersion(5) -> true 
 * 调用 isBadVersion(4) -> true
 * 所以，4 是第一个错误的版本。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1, bad = 1
 * 输出：1
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 
 * 
 */

// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        // 第一个错误版本
		let mut left:i32 = 1 as i32;
        // 
        let mut right:i32  = n ;

        //  1 2 3 4 5
        // mid = 3  
        // 如果三错误了，那么错误版本就在 12 里边。
        while left < right{
            // 先看中间是否异常
            let mid = (left+right)/2;
            let bad = self.isBadVersion(mid as i32);
            if bad {
                // 中间版本是错误版本，说明错误还在前面
                right = mid;
            }else{
                left = mid;
            }
        }

       return left as i32;
    }
}
// @lc code=end

