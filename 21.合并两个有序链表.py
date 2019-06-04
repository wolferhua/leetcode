#
# @lc app=leetcode.cn id=21 lang=python3
#
# [21] 合并两个有序链表
#
# https://leetcode-cn.com/problems/merge-two-sorted-lists/description/
#
# algorithms
# Easy (54.71%)
# Likes:    472
# Dislikes: 0
# Total Accepted:    72.7K
# Total Submissions: 132.9K
# Testcase Example:  '[1,2,4]\n[1,3,4]'
#
# 将两个有序链表合并为一个新的有序链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
#
# 示例：
#
# 输入：1->2->4, 1->3->4
# 输出：1->1->2->3->4->4
#
#
#
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None


class Solution:
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
       
        if not l1:
            return l2
        elif not l2:
            return l1
        # 确定头
        head = l1
        oth = l2
        if l2.val < l1.val:
            head = l2
            oth = l1
        # 确定排查
        n1 = head.next
        now = head
        n2 = oth
        while True:
            if n1 and n2:
                if n1.val > n2.val:
                    now.next = n2
                    n2 = n2.next
                else:
                    now.next = n1
                    n1 = n1.next
            elif n1:
                now.next = n1
                n1 = n1.next
            elif n2:
                now.next = n2
                n2 = n2.next
            else:
                break
            now = now.next
        return head
