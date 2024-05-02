#include <bits/stdc++.h>
using namespace std;

int main() {
  int n;
  double q, y;
  cin >> n;
  double d = 0.0;
  for (int i = 0; i < n; i++) {
    cin >> q >> y;
    d += q * y;
  }
  cout << d << endl;
  return 0;
}
