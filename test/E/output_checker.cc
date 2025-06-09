#include "testlib.h"
#include <vector>

using namespace std;
#define rep_(i, a_, b_, a, b, ...) for (int i = (a), lim##i = (b); i < lim##i; ++i)
#define rep(i, ...) rep_(i, __VA_ARGS__, __VA_ARGS__, 0, __VA_ARGS__) // rep(i, a): [0, a); rep(i, a, b): [a, b)

vector<vector<int>> rotate(const vector<vector<int>> &a)
{
  int n = a.size();
  vector<vector<int>> res(n, vector<int>(n));
  rep(i, n) rep(j, n) res[j][n - i - 1] = a[i][j];
  return res;
}

vector<int> observe(const vector<vector<int>> &a)
{
  int n = a.size();
  vector<int> res(n);
  rep(i, n)
  {
    rep(j, n)
    {
      if (a[i][j] != 0)
      {
        res[i] = a[i][j];
        break;
      }
    }
  }
  return res;
}

int main(int argc, char *argv[])
{
  setName("judge for Problem E");
  registerTestlibCmd(argc, argv);

  int case_num = 0;
  while (true)
  {
    // read testcase
    int n = inf.readInt();
    if (n == 0)
      break;
    vector<int> c(n);
    for (auto &e : c)
      e = inf.readInt();
    case_num++;

    // judge's answer
    string ja = ans.readToken();
    // participant's answer
    string pa = ouf.readToken();

    // check Yes/No
    quitif(ja != pa, _wa, "wrong answer for case %d - expected: '%s', found: '%s'", case_num, ja.c_str(), pa.c_str());
    if (ja == "No")
      continue;

    // read a
    vector<vector<int>> a_ans(n, vector<int>(n));
    for (auto &v : a_ans)
    {
      for (auto &e : v)
        e = ans.readInt(0, n, "a[i][j] of ans"); // check if a[i][j] is in [0, n]
    }
    vector<vector<int>> a(n, vector<int>(n));
    for (auto &v : a)
    {
      for (auto &e : v)
        e = ouf.readInt(0, n, "a[i][j]"); // check if a[i][j] is in [0, n]
    }

    // check if a satisfies the conditions
    rep(_, 4)
    {
      vector<int> b = observe(a);
      quitif(b != c, _wa, "wrong answer for case %d - the condition is not satisfied", case_num);
      a = rotate(a);
    }
  }
  // ouf.readEoln();
  // ouf.readEof();
  quitf(_ok, "correct answers for all %d cases", case_num);
}