#include <vector>
#include <string>
#include <list>
#include <algorithm>
#include <iostream>
#define N4_9 262144
#define N4_10 1048576

using namespace std;
class Solution {
    public:
        vector<string> findRepeatedDnaSequences(string s) {
            vector<string> result;
            int alpha[256] = {0};
            alpha['A'] = 0;
            alpha['C'] = 1;
            alpha['G'] = 2;
            alpha['T'] = 3;
            char alpha2[4] = {'A','C','G','T'};
            int str_len = s.length();
            vector<int> countv;
            if(str_len < 10) return result;
            int *string_num = new int[str_len]();
            for(int k = 0; k < 10; k++)
            {
                string_num[0] = string_num[0] * 4 + alpha[s[k]];
            }
            countv.push_back(string_num[0]);
            for(int i = 1; i < str_len - 9; i++)
            {
                string_num[i] = (string_num[i-1] % N4_9)*4 + alpha[s[i+9]];
                countv.push_back(string_num[i]);
            }
            sort(countv.begin(),countv.end());
            for(int i = 0; i < countv.size()-1; i++)
            {
                if(i == countv.size()-2)
                {
                    if(countv[i] == countv[i+1])
                    {
                        char t[11] = {0};
                        int tt = countv[i];
                        for(int i = 9; i >= 0; i--)
                        {
                            t[i] = alpha2[tt % 4];
                            tt /= 4;
                        }
                        result.push_back(string(t));
                    }
                }
                else
                {
                    if(countv[i] == countv[i+1] && countv[i] != countv[i+2])
                    {
                        char t[11] = {0};
                        int tt = countv[i];
                        for(int i = 9; i >= 0; i--)
                        {
                            t[i] = alpha2[tt % 4];
                            tt /= 4;
                        }
                        result.push_back(string(t));
                    }
                }
            }

            return result;
        }
};

int main(int argc, char const* argv[])
{
    Solution solution = Solution();
    //vector<string> result = solution.findRepeatedDnaSequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT");
    vector<string> result = solution.findRepeatedDnaSequences("AAAAAAAAAAAAA");
    for(auto it = result.begin(); it != result.end(); it++) cout << *it << endl;
    return 0;
}
