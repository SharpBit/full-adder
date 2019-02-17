use std::io;

fn main() {
    let mut num1 = String::with_capacity(4);
    println!("Enter a binary number:");
    io::stdin().read_line(&mut num1)
        .expect("Couldn't read line.");

    let mut num2 = String::with_capacity(4);
    println!("Enter another binary number:");
    io::stdin().read_line(&mut num2)
        .expect("Couldn't read line.");

    let mut digits = vec![vec![0; 3]; 5]; // [[a, b, cin], ..., [a, b, cin]]
    for (i, c) in num1.trim().chars().rev().collect::<String>().chars().enumerate() {
        digits[i][0] = int(c.to_string());
    }
    for (i, c) in num2.trim().chars().rev().collect::<String>().chars().enumerate() {
        digits[i][1] = int(c.to_string());
    }

    let mut ans = String::new();
    for i in 0 .. digits.len() {
        let j = &digits[i];
        let [sum, cout] = adder(j[0], j[1], j[2]);
        if i < 4 {
            digits[i+1][2] = cout;
        }
        ans += &sum.to_string();
    }
    ans = ans.chars().rev().collect::<String>();

    for i in 0..5 {
        if i == 0 {
            if &ans[i..i+1] == "1" {
                print!("{}", &ans[i..i+1]);
            }
        } else {
            print!("{}", &ans[i..i+1]);
        }
    }
    println!("");
}


fn adder(a: i32, b: i32, cin: i32) -> [i32; 2] {
    let half_sum = a ^ b;
    let sum = half_sum ^ cin;
    let cout = (a & b) | (cin & half_sum);
    [sum, cout]
}

fn int(string: String) -> i32 {
    string.parse::<i32>().unwrap()
}