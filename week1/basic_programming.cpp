#include <bits/stdc++.h>
using namespace std;

int main() {
  int n, t;
  cin >> n >> t;
  vector<int> a;
  for (int i = 0; i < n; i++) {
    int j;
    cin >> j;
    a.push_back(j);
  }
  if (t == 1) {
    cout << 7 << endl;
  } else if (t == 2) {
    if (a[0] > a[1]) {
      cout << "Bigger" << endl;
    } else if (a[0] == a[1]) {
      cout << "Equal" << endl;
    } else {
      cout << "Smaller" << endl;
    }
  } else if (t == 3) {
    vector<int> v = {a[0], a[1], a[2]};
    sort(v.begin(), v.end());
    cout << v[1] << endl;
  } else if (t == 4) {
    int s = 0;
    for (int x: a) {
      s += x;
    }
    cout << s << endl;
  } else if (t == 5) {
    int s = 0;
    for (int x: a) {
      if (x % 2 == 0) {
        s += x;
      }
    }
    cout << s << endl;
  } else if (t == 6) {
    string s = "";
    for (int x: a) {
      int y = x % 26;
      s += (char)((int)('a') + y);
    }
    cout << s << endl;
  } else if (t == 7) {
    vector<int> done;
    int i = 0;
    int j;
    done.push_back(0);
    while (1) {
      j = a[i];
      done.push_back(i);
      if (find(done.begin(), done.end(), j) != done.end()) {
        cout << "Cyclic" << endl;
        break;
      } else if (i > n - 1 || i < 0) {
        cout << "Out" << endl;
        break;
      } else if (i == n - 1) {
        cout << "Done" << endl;
        break;
      } else {
        done.push_back(j);
        i = j;
      }
    }
  }
  return 0;
}
