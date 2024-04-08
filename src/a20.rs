
#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2024-04-08 19:49-20:47 (58min)
    // 最長共通部分列問題 (Longest Common Subsequence, LCS)
    // 2つの文字列 X[1,...,m] , Y[1,…,n] において、双方に現れる部分文字列の中で最長の文字列と定義されています。 部分列は連続している必要はありませんが，順序を変更してはいけません。
    // https://www.cs.t-kougei.ac.jp/SSys/LCS.htm#:~:text=LCS%20(%E6%9C%80%E9%95%B7%E5%85%B1%E9%80%9A%E9%83%A8%E5%88%86%E5%88%97,%E3%81%A6%E3%81%AF%E3%81%84%E3%81%91%E3%81%BE%E3%81%9B%E3%82%93%E3%80%82
    input! {
        s: Chars,
        t: Chars
    }

    // dp[i][j] := Sをi文字目まで、Tをj文字目まで見たときの、最大部分文字列数
    // 問題の核：dp[i][j]までが既知のとき、初見の文字はs[i+1]とt[j+1]であり、
    //  s[i+1] == t[j+1] のとき、dp[i+1][j+1] => dp[i][j] + 1 となる。
    // あとは遷移の漏れを考える。
    // (i,j)=(0,..)と(..,0)は、遷移元が無い(左上のマスが存在しない)
    let mut dp = vec![vec![0; t.len()]; s.len()];

    if s[0] == t[0] {
        dp[0][0] = 1;
    }
    for i in 1..s.len() {
        dp[i][0] = max(dp[i][0], dp[i-1][0]);
        if s[i] == t[0] {
            dp[i][0] = 1;
        }
    }
    for i in 1..t.len() {
        dp[0][i] = max(dp[0][i], dp[0][i-1]);
        if s[0] == t[i] {
            dp[0][i] = 1;
        }
    }
    
    for i in 0..s.len() {
        for j in 0..t.len() {
            if i+1 < s.len() &&  j+1 < t.len() {
                dp[i+1][j+1] = max(dp[i+1][j+1], dp[i][j]);
                dp[i+1][j+1] = max(dp[i+1][j+1], dp[i][j+1]);
                dp[i+1][j+1] = max(dp[i+1][j+1], dp[i+1][j]);
                if s[i+1] == t[j+1] {
                    dp[i+1][j+1] = max(dp[i+1][j+1], dp[i][j] + 1);
                }
            }
        }
    }
    let ans = dp[s.len()-1][t.len()-1];
    println!("{}", ans);
    // dp.print_2d_array();

}

// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
    fn print_2d_array_transposed(&self);
    fn print_2d_array_transposed_with_name(&self, name: &str);
}
impl<T: std::fmt::Debug> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                print!("{:?} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_with_name(&self, name: &str) {
        println!("-=-=-=-= {} -=-=-=-=", name);
        self.print_2d_array();
        println!("-=-=-=-=-=-=-=-=");
    }
    fn print_2d_array_transposed(&self) {
        for x in 0..self[0].len() {
            print!("{:02}: ", x);
            for y in 0..self.len() {
                print!("{:?} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_transposed_with_name(&self, name: &str) {
        println!("-=-=-=-= transposed {} -=-=-=-=", name);
        self.print_2d_array_transposed();
        println!("-=-=-=-=-=-=-=-=");
    }
}