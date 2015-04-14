# -*- coding: utf-8 -*-
class Solution:
    """
    https://leetcode.com/problems/jump-game-ii

    Given an array of non-negative integers, you are initially positioned at the first index of the array.

    Each element in the array represents your maximum jump length at that position.

    Your goal is to reach the last index in the minimum number of jumps.

    For example:
        Given array A = [2,3,1,1,4]

        The minimum number of jumps to reach the last index is 2. (Jump 1 step from index 0 to 1, then 3 steps to the last index.)
    """
    # @param A, a list of integers
    # @return an integer
    def jump(self, A):
        """
        2  3  1  1  4
        *
        -> |  | 人在 A[0] 最远到 A[1],A[2]
           *  *
           ->    |  |  人在 A[1] - A[2], 最远可到 A[1+3]
                 *  *
        """
        start, end, max_reach = 0,0,0
        count = 0
        while max_reach < len(A)-1:
            for i in range(start,end+1):
                max_reach = max(max_reach, i+A[i])
            count += 1
            start, end = end+1, max_reach
        return count

if __name__ == '__main__':
    A = [2,3,1,1,4]
    B = [2]
    C = [9,4,5,4,1,8,1,2,0,7,8,7,0,6,6,1,1,2,5,0,9,8,4,7,9,6,8,1,4,0,8,5,5,3,9,8,1,2,2,3,0,1,3,2,7,9,3,0,1]
    D = [8,2,4,4,4,9,5,2,5,8,8,0,8,6,9,1,1,6,3,5,1,2,6,6,0,4,8,6,0,3,2,8,7,6,5,1,7,0,3,4,8,3,5,9,0,4,0,1,0,5,9,2,0,7,0,2,1,0,8,2,5,1,2,3,9,7,4,7,0,0,1,8,5,6,7,5,1,9,9,3,5,0,7,5]
    E = []
    with open("input.txt") as f:
        F = eval(f.read())
    with open("input2.txt") as f:
        G = eval(f.read())
    solution = Solution()
    print(solution.jump(A))
    print(solution.jump(B))
    print(solution.jump(C))
    print(solution.jump(D))
    print(solution.jump(E))
    print(solution.jump(F))
    print(solution.jump(G))
