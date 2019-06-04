#
# @lc app=leetcode.cn id=20 lang=python3
#
# [20] 有效的括号
#


class Solution:
    def isValid(self, s: str) -> bool:
        # 通过栈来实现，先将 `(、{、[` 压入栈，当遇到 `]、}、)` 出栈，并进行对比。如果匹配则通过。
        # 声明栈
        stack = []
        # 结束值
        match = {')': '(', ']': '[', '}': '{'}
        # 便利s
        for c in s: 
            if c in match:
                # 判断栈是否为空
                if not stack:
                    return False
                # 出栈
                sc = stack.pop()
                if sc != match[c]:
                    return False
            else:
                # 加入栈
                stack.append(c) 
        return not stack # 如果栈空了，表示所有数据均已完全匹配，则是正常的括号。
