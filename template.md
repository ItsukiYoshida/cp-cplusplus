
-----

# ICPC/競技プログラミング用 C++テンプレート解説

このドキュメントは，競技プログラミング用のC++テンプレートについての説明です．

## 1\. コンパイルオプション (Pragma)

```cpp
// #pragma GCC target("avx2")
// #pragma GCC optimize("O3")
// #pragma GCC optimize("unroll-loops")
```

コードの先頭にあるこれらの行は，GCCコンパイラに対する最適化指示です．

  - **`target("avx2")`**: AVX2命令セットの使用を許可し，特定のベクトル演算を高速化します．
  - **`optimize("O3")`**: 最大最適化を有効にします．
  - **`optimize("unroll-loops")`**: ループ展開を強制し，ループのオーバーヘッドを削減します．

デフォルトではコメントアウトされています．必要に応じて自己責任で設定してください．

-----

## 2\. ヘッダーと名前空間

```cpp
#include <bits/extc++.h>
using namespace std;
using namespace __gnu_pbds;
```

  - **`#include <bits/extc++.h>`**: GCC拡張を含む，競技プログラミングで使われるほぼ全ての標準ライブラリとデータ構造を一括でインクルードします．
  - **`using namespace std;`**: `std::`の記述を省略します．
  - **`using namespace __gnu_pbds;`**: 後述するPBDS（Policy-Based Data Structures）を使用するために必要です．

-----

## 3\. 型エイリアスと定数

コードの可読性と記述の簡潔さのために，よく使う型や定数に別名を付けています．

  - **エイリアス**:
      - `ll`: `long long`
      - `pii`: `pair<int, int>`
      - `vi`: `vector<int>`
      - `vvi`: `vector<vector<int>>`
      - `vll`: `vector<ll>`
      - `vvll`: `vector<vector<ll>>`
  - **定数**:
      - `INF`: `1e9` (int型の巨大数)
      - `LINF`: `1e18` (long long型の巨大数)
      - `MOD`: `1e9+7` または `998244353` (剰余演算で頻出の素数)

-----

## 4\. マクロとヘルパー関数

いくつかの便利なマクロと関数が定義されています．

  - **`#define ALL(x) x.begin(), x.end()`**: コンテナの先頭から末尾までのイテレータ範囲を返します．`sort(ALL(v));` のように使います．
  - **`chmax(a, b)` / `chmin(a, b)`**: `a`を`b`で更新する際に，より大きい/小さい値であれば更新し`true`を返します．`if (a < b) a = b;`よりも簡潔に書けます．

-----

## 5\. Policy-Based Data Structures (PBDS)

`__gnu_pbds`名前空間で提供される，標準ライブラリにはない高性能なデータ構造です．

### `ordered_set`

```cpp
template <typename T>
using ordered_set = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;
```

赤黒木をベースとした平衡二分探索木で，順序統計量に関する操作が高速に行えます．

  - **概要**: `std::set`の機能に加え，「自分より小さい要素は何個あるか」「k番目に小さい要素は何か」というクエリに答えられます．
  - **主なメソッド**:
      - `insert(x)`: 要素`x`を挿入．計算量は $O(\log N)$ です．
      - `erase(x)`: 要素`x`を削除．計算量は $O(\log N)$ です．
      - `order_of_key(k)`: `k`より小さい要素の数を返します．計算量は $O(\log N)$ です．
      - `find_by_order(k)`: 0-indexedで`k`番目に小さい要素へのイテレータを返します．計算量は $O(\log N)$ です．

### `trie_tree`

```cpp
using trie_tree = trie<string, null_type, trie_string_access_traits<>, pat_trie_tag, trie_prefix_search_node_update>;
```

PATRICIA TrieをベースとしたTrie木です．文字列集合の管理や前方一致検索に適しています．

  - **概要**: 多数の文字列の中から特定のプレフィックスを持つものを高速に検索できます．
  - **主なメソッド**:
      - `insert(s)`: 文字列`s`を挿入．計算量は $O(|s|)$ です．
      - `erase(s)`: 文字列`s`を削除．計算量は $O(|s|)$ です．
      - `prefix_range(s)`: `s`をプレフィックスに持つ全ての要素を指すイテレータのペア `(first, last)` を返します．計算量は $O(|s|)$ です．

-----

## 6\. カスタムデータ構造

### `compress` (座標圧縮)

```cpp
template<typename T>
vector<int> compress(const vector<T>& v);
```

  - **概要**: 数列などに含まれる値の大小関係を保ったまま，それらを `0, 1, 2, ...` といった小さい整数に置き換えます．
  - **使い方**: `compress(vec)` のように，圧縮したい値が入った`vector`を渡します．元の`vector`と同じサイズの，圧縮後の値が入った`vector`が返ります．
  - **計算量**: $O(N \log N)$

### `SparseTable`

  - **概要**: 静的な配列に対し，区間に対するクエリ（最小値，最大値，GCDなど）を高速に処理します．適用できる演算は**冪等性**を持つ必要があります．
  - **計算量**:
      - **構築**: $O(N \log N)$
      - **クエリ**: $O(1)$
  - **主なメソッド**:
      - **コンストラクタ `SparseTable(v, f)`**: 元となる `vector` `v` と，区間の値をマージする関数 `f` で初期化します．
      - **`query(l, r)`**: 半開区間 `[l, r)` に対するクエリ結果を返します．

### `LowestCommonAncestor` (LCA)

  - **概要**: 木構造における2頂点`u`, `v`の最も近い共通祖先（LCA）を求めます．ダブリングという手法を用いています．
  - **計算量**:
      - **構築**: $O(N \log N)$
      - **クエリ**: $O(\log N)$
  - **主なメソッド**:
      - **コンストラクタ `LowestCommonAncestor(G, root)`**: 隣接リスト表現のグラフ `G` と，木の根 `root` で初期化します．
      - **`get(u, v)`**: 2頂点 `u`, `v` のLCAの頂点番号を返します．
      - **`dist(u, v)`**: 2頂点 `u`, `v` 間の距離（辺の数）を返します．

-----

## 7\. デバッグ

```cpp
#ifdef LOCAL 
#include <debug.hpp>
#else
#define debug(...) (void)0
#define fdebug(...) (void)0
#endif
```

`debug.h`ではユニバーサルな出力ストリームの演算子オーバーロードを実装しています．基本的には手動で作成する必要はありませんが，必要に応じて左シフト演算子をオーバーロードしてください．
標準コンテナに対しては動作することを確認済みです．
<!--
これは嘘です．実際にはvector, set, mapに対して動作することのみを確認しています．
が，多分全部動くと思います．チェックめんどくさいからね．

TODO
[x] vector
[x] set
[x] map
[ ] unordered_set
[ ] unordered_map
[ ] list
[ ] hoge
[ ] hoge
[ ] hoge
[ ] hoge
[ ] hoge
[ ] hoge
[ ] hoge
-->

LOCAL環境のみで実行される`debug`関数を定義しています．
これをソースコードに挟むことで，標準エラー出力にDebug用の出力を行います．
### `debug`
 - **概要**: 可変長引数を受け取り，各引数のその時点での中身を`debug`が実行されたファイル，行，変数名とともに要素をすべて出力します．
 各引数ごとに開業されて出力されます．出力された文字のうち実行されたファイル，行が黄色，変数名が赤色，要素が白/黒色で出力されます．
 - **サンプル コード**
 ```cpp
 vector<int> v(n);
 iota(v.begin(), v.end(), 0);
 set<int> st;
 map<int, int> mp;
 for (auto &&x : v) {
     st.insert(x);
     mp[x] = x << 1;
 }
 debug(v);
 debug(st);
 debug(mp);
 debug(v, st, mp);
 ```
 - **出力**
 ```bash
 === debug ===
[path/to/a.cpp:162] v: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
=============
=== debug ===
[path/to//a.cpp:163] st: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
=============
=== debug ===
[path/to//a.cpp:164] mp: {0: 0, 1: 2, 2: 4, 3: 6, 4: 8, 5: 10, 6: 12, 7: 14, 8: 16, 9: 18}
=============
=== debug ===
[path/to//a.cpp:165] v: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                      st: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                      mp: {0: 0, 1: 2, 2: 4, 3: 6, 4: 8, 5: 10, 6: 12, 7: 14, 8: 16, 9: 18}
 ```
### `fdebug`
 - **概要**: おおむね`debug`と同じですが，変数名の部分を手動で設定するようになっています．それに伴い，引数の数は偶数である必要があります．
 この関数が定義されている理由としては，`debug`では`v[i]`をfor分で与えた際に`i`が評価されないためです．
 また補助用マクロとして`L(stream)`を定義してあります．これにより変数名の設定を行うことができます．このマクロは`stream`を処理して`string`を生成しています．そのため，`string`を名前として用いる場合は使う必要はありません．
 - **サンプル コード**
 ```cpp
 vector<int> v(n);
 iota(v.begin(), v.end(), 0);
 for (int i = 0; i < n; i++) {
	// debug(v[i]);
    fdebug(L("v[" << i << "]"), v[i]);
 }
 fdebug(L("This is set"), st, "this is map", mp);
 ```
 - **出力**
 ```bash
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:168] v[0]: 0
=============
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:168] v[1]: 1
=============
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:168] v[2]: 2
=============
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:168] v[3]: 3
=============
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:168] v[4]: 4
=============
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:168] v[5]: 5
=============
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:168] v[6]: 6
=============
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:168] v[7]: 7
=============
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:168] v[8]: 8
=============
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:168] v[9]: 9
=============
=== debug ===
[/home/cat/repos/cp/main/src/a.cpp:170] This is set: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                                        this is map: {0: 0, 1: 2, 2: 4, 3: 6, 4: 8, 5: 10, 6: 12, 7: 14, 8: 16, 9: 18}
=============
 ```

-----

## 8\. プログラムの基本構造

```cpp
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
    // 問題ごとのロジックをここに書く
}
```

  - **`main`関数**:
      - `ios_base::sync_with_stdio(false); cin.tie(nullptr);` は，C++の標準入出力(`cin`, `cout`)を高速化するためのおまじないです．
      - `cout << fixed << setprecision(15);` は，浮動小数点数を出力する際の精度を15桁に固定します．
      - `while(cin >> n, n)` ループは，ICPCでよくある「入力が0で終わる」形式の問題に対応しています．
  - **`solve`関数**:
      - 各テストケースの具体的な処理は，この`solve`関数内に記述します．