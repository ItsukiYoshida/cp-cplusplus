#include <vector>
#include "testlib.h"

using namespace std;

const int MAX_VAL = 1000000000;

bool compare_vectors(int n, vector<int> a, vector<int> b){
    sort(a.begin(), a.end());
    sort(b.begin(), b.end());
    for(int i = 0; i < n; i++){
        if(a[i] != b[i]) return false;
    }
    return true;
}

int main(int argc, char *argv[]) {
    setName("judge for Problem G");
    registerTestlibCmd(argc, argv);

    int case_num = 0;
    while(true){
        // read testcase
        int n = inf.readInt();
        if(n == 0) break;
        case_num++;

        vector<int> s(n);
        for(int i = 0; i < n; i++) s[i] = inf.readInt();

        // judge's answer
        string ja = ans.readWord();
        // participant's answer
        string pa = ouf.readWord();

        if(ja != pa){
            quitf(_wa, "wrong answer for case %d - expected: '%s', found: '%s'", case_num, ja.c_str(), pa.c_str());
        }

        if(ja == "No") continue;

        // read a, b from judge's answer (not used in checker)
        vector<int> a_ans(n), b_ans(n);
        for(int i = 0; i < n; i++) a_ans[i] = ans.readInt();
        for(int i = 0; i < n; i++) b_ans[i] = ans.readInt();

        // read a, b from participant's answer
        vector<int> a(n), b(n);
        for(int i = 0; i < n; i++) a[i] = ouf.readInt(-MAX_VAL, MAX_VAL);
        for(int i = 0; i < n; i++) b[i] = ouf.readInt(-MAX_VAL, MAX_VAL);

        // check if a, b satisfies the conditions
        for(int i = 0; i < n; i++){
            if(a[i] + b[i] != s[i]){
                quitf(_wa, "wrong answer for case %d - the sum unmatches for i = %d", case_num, i + 1);
            }
        }

        if(!compare_vectors(n, a, b)){
            quitf(_wa, "wrong answer for case %d - vector b is not a permutation of vector a", case_num);
        }
    }

    quitf(_ok, "correct answers for %d cases", case_num);
}
