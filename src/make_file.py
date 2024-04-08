"""90題分のrsファイル作成とCargo.tomlの編集([[bin]]の追加)を行う"""

rs_file_contents = """
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
    input! {
        
    }
}"""



cargo_toml_content = '''
[[bin]]
name = "{}{:0=2}"
path = "src/{}{:0=2}.rs"
'''

kind_num_map = {
    "a": 77,
    "b": 69,
    "c": 20,
}


for kind, num in kind_num_map.items():
    for i in range(1, num + 1):
        filename = "{}{:0=2}.rs".format(kind, i)

        # .rsファイルの作成
        with open(filename, mode="w") as f:
            f.write(rs_file_contents)
        
        # Cargo.tomlの編集
        with open("../Cargo.toml", mode="a") as f:
            f.write(cargo_toml_content.format(kind, i, kind, i))