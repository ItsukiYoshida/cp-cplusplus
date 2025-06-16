// https://atcoder.jp/contests/abc292/tasks/abc292_d
#include <bits/extc++.h>
#include <atcoder/all>

using namespace std;
using namespace atcoder;

int main() {
	int n, m; cin >> n >> m;
	dsu uf(n);
	vector<int> ver(n);
	for (int i = 0; i < m; i++) {
		int u, v; cin >> u >> v;
		u--;v--;
		if (!uf.same(u, v)) {
			int uv = ver[uf.leader(u)];
			int vv = ver[uf.leader(v)];
			uf.merge(u, v);
			ver[uf.leader(u)] = uv + vv + 1;
		} else {
			ver[uf.leader(u)]++;
		}
	}
	auto leaders = uf.groups();
	for (auto &&v : leaders) {
		if (uf.size(v[0]) != ver[uf.leader(v[0])]) {
			puts("No");
			return 0;
		}
	}
	puts("Yes");
}