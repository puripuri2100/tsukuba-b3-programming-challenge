#include <bits/stdc++.h>
using namespace std;

int main() {
  int n, k;
  int is_head = 1;
  while (1) {
    cin >> n;
    if (n == 0) {
      return 0;
    }
    if (is_head) {
      is_head = 0;
    } else {
      cout << "" << endl;
    }
    cin >> k;
    vector<int> w(n);
    vector<int> l(n);
    for (int i = 0; i < ((k * n * (n - 1)) / 2); i++) {
      int p1, p2;
      string m1, m2;
      cin >> p1 >> m1 >> p2 >> m2;
      if (m1 == "rock") {
        if (m2 == "paper") {
          l[p1 - 1] += 1;
          w[p2 - 1] += 1;
        } else if (m2 == "scissors") {
          w[p1 - 1] += 1;
          l[p2 - 1] += 1;
        }
      } else if (m1 == "paper") {
        if (m2 == "scissors") {
          l[p1 - 1] += 1;
          w[p2 - 1] += 1;
        } else if (m2 == "rock") {
          w[p1 - 1] += 1;
          l[p2 - 1] += 1;
        }
      } else { // m1 == scissors
        if (m2 == "rock") {
          l[p1 - 1] += 1;
          w[p2 - 1] += 1;
        } else if (m2 == "paper") {
          w[p1 - 1] += 1;
          l[p2 - 1] += 1;
        }
      }
    }
    for (int p = 0; p < n; p++) {
      if ((w[p] + l[p]) == 0) {
        cout << "-" << endl;
      } else {
        double ans = (double)w[p] / (((double)w[p]) + ((double)l[p]));
        cout << fixed << setprecision(3) << ans << endl;
      }
    }
  }
  return 0;
}
