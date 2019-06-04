#
# @lc app=leetcode.cn id=20 lang=python3
#
# [20] 有效的括号
#
class Solution:
    def isValid(self, s: str) -> bool:
        # 通过栈来实现，先将 `(、{、[` 压入栈，当遇到 `]、}、)` 出栈，并进行对比。如果匹配则通过。

