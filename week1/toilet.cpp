#include <bits/stdc++.h>
using namespace std;

int main() {
  string str;
  cin >> str;
  vector<char> data(str.begin(), str.end());
  int a1 = 0;
  int a2 = 0;
  int a3 = 0;
  char tmp_a1 = data[0];
  char tmp_a2 = data[0];
  char tmp = data[0];
  data.erase(data.begin());
  for (char c: data) {
    if (tmp != c) {
      a3 += 1;
    }
    if (c == 'D') {
      a1 += 1;
      if (tmp_a1 == 'U') {
        a1 += 1;
      }
      if (tmp_a2 == 'U') {
        a2 += 1;
      }
    }
    if (c == 'U') {
      a2 += 1;
      if (tmp_a1 == 'D') {
        a1 += 1;
      }
      if (tmp_a2 == 'D') {
        a2 += 1;
      }
    }
    tmp_a1 = 'U';
    tmp_a2 = 'D';
    tmp = c;
  }
  if (tmp_a2 == 'U') {
    a2 += 1;
  }
  if (tmp_a1 == 'D') {
    a1 += 1;
  }
  cout << a1 << endl;
  cout << a2 << endl;
  cout << a3 << endl;
  return 0;
}
