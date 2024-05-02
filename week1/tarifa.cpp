#include <bits/stdc++.h>
using namespace std;

int main() {
  int x, n, j;
  cin >> x;
  cin >> n;
  int s = 0;
  for (int i = 0; i < n; i++) {
    cin >> j;
    s += j;
  }
  int a = (x * (n + 1)) - s;
  cout << a << endl;
  return 0;
}
