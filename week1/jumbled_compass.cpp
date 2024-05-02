#include <bits/stdc++.h>
using namespace std;

int main() {
  int n1, n2;
  cin >> n1;
  cin >> n2;
  int x = n2 - n1; // 動く角樋（愚直）
  if (x < -180) {
    cout << 360 + x << endl;
  } else if (x > 180) {
    cout << x - 360 << endl;
  } else if (x == -180) {
    cout << 180 << endl;
  } else {
    cout << x << endl;
  }
  return 0;
}
