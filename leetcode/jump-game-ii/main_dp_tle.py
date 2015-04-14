class Solution:
    """
    DP
    TLE with G
    """
    # @param A, a list of integers
    # @return an integer
    def jump(self, A):
        self.A = A
        self.paylist = [len(A)+100] * (len(A)-1) + [0]
        self.dp()
        return self.paylist[0]

    def dp(self):
        for pos in xrange(len(self.A)-2,-1,-1):
            for step in xrange(self.A[pos],0,-1):
                if pos + step >= len(self.A):
                    self.paylist[pos] = 1
                    break
                tmp = self.paylist[pos + step] + 1
                if tmp < self.paylist[pos]:
                    self.paylist[pos] = tmp


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
