// #pragma GCC target("avx2")
// #pragma GCC optimize("O3")
// #pragma GCC optimize("unroll-loops") //この辺ローカル環境ではCMakeとコンフリクトしてうまく動かない．

#include <bits/extc++.h>
using namespace std;
using ll = long long;
using namespace __gnu_pbds;

template <typename T>
using ordered_set = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;
using trie_tree = trie<string, null_type, trie_string_access_traits<>, pat_trie_tag, trie_prefix_search_node_update>;

using pii = pair<int, int>;
using vi = vector<int>;
using vvi = vector<vector<int>>;
using vll = vector<ll>;
using vvll = vector<vector<ll>>;

constexpr int INF = 1e9;
constexpr ll LINF = 1e18;
constexpr int MOD = 1e9+7;
// constexpr int MOD = 998244353;
#define ALL(x) x.begin(), x.end()

#ifdef LOCAL 
#include <debug.hpp>
#else
#define debug(...) (void)0
#define fdebug(...) (void)0
#endif

template<class T>
bool chmax(T &a, const T &b) {
	if (a < b) {
		a = b;
		return true;
	}
	return false;
}
template<class T>
bool chmin(T &a, const T &b) {
	if (a > b) {
		a = b;
		return true;
	}
	return false;
}
template<typename T>
vector<int> compress(const vector<T>& v) {
    int n = v.size();
    vector<T> sorted_v = v;
    sort(sorted_v.begin(), sorted_v.end());
    sorted_v.erase(unique(sorted_v.begin(), sorted_v.end()), sorted_v.end());
    vector<int> res(n);
    for (int i = 0; i < n; ++i) {
        res[i] = lower_bound(sorted_v.begin(), sorted_v.end(), v[i]) - sorted_v.begin();
    }
    return res;
}
template <typename T>
struct SparseTable {
    using F = function<T(T, T)>;
    vector<vector<T>> table;
    vector<int> log_table;
    F f;

    SparseTable(const vector<T>& v, F f) : f(f) {
        int n = v.size();
        log_table.assign(n + 1, 0);
        for (int i = 2; i <= n; i++) {
            log_table[i] = log_table[i / 2] + 1;
        }
        table.assign(log_table[n] + 1, vector<T>(n));
        table[0] = v;
        for (int i = 1; i < table.size(); i++) {
            for (int j = 0; j + (1 << i) <= n; j++) {
                table[i][j] = f(table[i - 1][j], table[i - 1][j + (1 << (i - 1))]);
            }
        }
    }

    // [l, r) の区間のクエリ
    T query(int l, int r) {
        int len = r - l;
        int k = log_table[len];
        return f(table[k][l], table[k][r - (1 << k)]);
    }
};
struct LowestCommonAncestor {
    const int n;
    const int log_n;
    vector<vector<int>> parent;
    vector<int> depth;
    const vector<vector<int>>& G;

    LowestCommonAncestor(const vector<vector<int>>& G, int root = 0)
        : n(G.size()), log_n(log2(n) + 1), parent(log_n, vector<int>(n)), depth(n), G(G) {
        dfs(root, -1, 0);
        for (int k = 0; k + 1 < log_n; k++) {
            for (int v = 0; v < n; v++) {
                if (parent[k][v] < 0) parent[k + 1][v] = -1;
                else parent[k + 1][v] = parent[k][parent[k][v]];
            }
        }
    }

    void dfs(int v, int p, int d) {
        parent[0][v] = p;
        depth[v] = d;
        for (int u : G[v]) {
            if (u != p) dfs(u, v, d + 1);
        }
    }

    int get(int u, int v) {
        if (depth[u] > depth[v]) swap(u, v);
        for (int k = 0; k < log_n; k++) {
            if ((depth[v] - depth[u]) >> k & 1) {
                v = parent[k][v];
            }
        }
        if (u == v) return u;
        for (int k = log_n - 1; k >= 0; k--) {
            if (parent[k][u] != parent[k][v]) {
                u = parent[k][u];
                v = parent[k][v];
            }
        }
        return parent[0][u];
    }
    
    int dist(int u, int v) {
        int lca = get(u, v);
        return depth[u] + depth[v] - 2 * depth[lca];
    }
};
void solve(const int n);

signed main() {
	ios_base::sync_with_stdio(false);
	cin.tie(nullptr);
	cout << fixed << setprecision(15);
	int n;
	while(cin >> n, n) {
		solve(n);
	}
}

void solve(const int n) {
}