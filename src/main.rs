// -*- coding:utf-8-unix -*-

use proconio::input;
use std::process;
// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    }
    let mut ans = x - a;
    let mut ans_times = 0;
    if d == 0 {
        if ans > 0 {
            println!("{}", ans);
        } else {
            println!("{}", -ans);
        }
        process::exit(0);
    }
    if ans > d * (n - 1) && d > 0 {
        ans_times = ans - d * (n - 1);
    } else if ans < 0 && d > 0 {
        ans_times = -ans;
    } else if ans < d * (n - 1) && d < 0 {
        ans_times = -(ans - d * (n - 1));
    } else if ans > 0 && d < 0 {
        ans_times = ans;
    } else {
        if d > 0 {
            if ans > 0 {
                ans_times = ans % d;
                if ans_times > d / 2 {
                    ans_times = d - ans_times;
                }
            } else {
                ans_times = (-ans) % d;
                if ans_times > d / 2 {
                    ans_times = d - ans_times;
                }
            }
        } else {
            if ans > 0 {
                ans_times = ans % (-d);
                if ans_times > (-d) / 2 {
                    ans_times = (-d) - ans_times;
                }
            } else {
                ans_times = (-ans) % (-d);
                if ans_times > (-d) / 2 {
                    ans_times = (-d) - ans_times;
                }
            }
        }
    }
    println!("{}", ans_times);
}
