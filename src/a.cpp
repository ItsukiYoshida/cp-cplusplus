#include <iostream>
using namespace std;

int main() {
	string s;
	getline(cin, s);
	if (s == "Hello, ICPC") {
		cout << "Hello, Mistake" << endl;
		return 0;
	}
	cout << s << endl;
}