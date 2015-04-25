#include <vector>
#include <string>
#include <set>
#include <iostream>

using namespace std;
class Solution {
    public:
        vector<string> findRepeatedDnaSequences(string s) {
            if(s.length() < 10)return vector<string>();
            set<string> s1,res;
            for(int i = 0; i < s.length()-9; i++)
            {
                pair<std::set<string>::iterator,bool> ret;
                ret = s1.insert(s.substr(i,10));
                if(ret.second == false)
                    res.insert(s.substr(i,10));
            }
            return vector<string>(res.begin(),res.end());
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
