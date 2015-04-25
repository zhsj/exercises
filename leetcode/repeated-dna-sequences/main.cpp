#include <vector>
#include <string>
#include <iostream>
#define N4_9 262144
#define N4_10 1048576

using namespace std;

/// https://leetcode.com/problems/repeated-dna-sequences/
///
/// All DNA is composed of a series of nucleotides abbreviated as A, C, G, and T, for example: "ACGAATTCCG". When studying DNA, it is sometimes useful to identify repeated sequences within the DNA.
///
/// Write a function to find all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule.
///
/// For example,
///
/// Given s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT",
///
/// Return:
/// ["AAAAACCCCC", "CCCCCAAAAA"].
class Solution {
    public:
        /// 因为只有4个字母，则可以看成4进制数，
        /// 对每一个10个字母的子串计算出其对应的数值。
        ///
        /// 然后用一个计数的数组（用空间换时间）来统计每个数字出现的频率。
        /// 一开始超内存了，后来把数组从 int[N4_10] 改为 char[N4_10] 就通过了。
        vector<string> findRepeatedDnaSequences(string s) {
            vector<string> result;
            char count[N4_10] = {0};
            int alpha[256] = {0};
            alpha['A'] = 0;
            alpha['C'] = 1;
            alpha['G'] = 2;
            alpha['T'] = 3;
            int str_len = s.length();
            if(str_len < 10) return result;

            int *string_num = new int[str_len]();
            for(int k = 0; k < 10; k++)
            {
                string_num[0] = string_num[0] * 4 + alpha[s[k]];
            }
            count[string_num[0]]++;
            for(int i = 1; i < str_len - 9; i++)
            {
                string_num[i] = (string_num[i-1] % N4_9)*4 + alpha[s[i+9]];
                if(count[string_num[i]] == 0)
                {
                    count[string_num[i]]++;
                }
                else if(count[string_num[i]] == 1)
                {
                    count[string_num[i]]++;
                    result.push_back(s.substr(i,10));
                }
            }

            return result;
        }
};

int main(int argc, char const* argv[])
{
    Solution solution = Solution();
    vector<string> result = solution.findRepeatedDnaSequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT");
    //vector<string> result = solution.findRepeatedDnaSequences("AAAAAAAAAAAAA");
    for(auto it = result.begin(); it != result.end(); it++) cout << *it << endl;
    return 0;
}
