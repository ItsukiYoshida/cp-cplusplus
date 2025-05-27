#include <iostream>
using namespace std;

int main() {
	int n;
	while(cin >> n) {
		if (n != 0) {
			int ans = 0;
			for (int i = 0; i < n; i++) {
				int x; cin >> x;
				ans += x;
			}
			cout << ans << endl;
		}
	}
}