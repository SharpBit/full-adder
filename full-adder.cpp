#include <cstdlib>
#include <string>
#include <iostream>
#include <algorithm>
#include <vector>

int* adder(int a, int b, int cin) {
    static int res[2];
    int half_sum = a ^ b;
    res[0] = half_sum ^ cin;
    res[1] = (a & b) | (cin & half_sum);
    return res;
}

int main() {
    std::string num1;
    std::string num2;

    std::cout << "Enter a binary number: ";
    std::cin >> num1;
    std::cout << "Enter another binary number: ";
    std::cin >> num2;

    std::vector<std::vector<int>> digits(5, std::vector<int>(3, 0)); // [[a, b, cin], ..., [a, b, cin]]
    std::reverse(num1.begin(), num1.end());
    std::reverse(num2.begin(), num2.end());
    for (int i=0; i<4; i++) {
        digits.at(i).at(0) = std::stoi(std::string(1, num1[i]));
    }
    for (int i=0; i<4; i++) {
        digits.at(i).at(1) = std::stoi(std::string(1, num2[i]));
    }

    std::string ans;
    for (int i=0; i<5; i++) {
        int* res = adder(digits.at(i).at(0), digits.at(i).at(1), digits.at(i).at(2));
        if (i < 4) {
            digits.at(i+1).at(2) = res[1];
        }
        ans += std::to_string(res[0]);
    }
    std::reverse(ans.begin(), ans.end());
    if (ans[0] == '0') {
        // better way to do this?
        std::cout << ans[1];
        std::cout << ans[2];
        std::cout << ans[3];
        std::cout << ans[4];
        std::cout << std::endl;
    } else {
        std::cout << ans << std::endl;
    }

    return 0;
}
