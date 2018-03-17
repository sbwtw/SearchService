
use std::cmp::max;

pub fn lcs(s1: &Vec<char>, s2: &Vec<char>) -> Vec<usize> {
    let len1 = s1.len();
    let len2 = s2.len();

    let mut data: Vec<Vec<usize>> = Vec::with_capacity(len1);
    let mut inner = Vec::with_capacity(len2);
    for _ in 0..len2 { inner.push(0); }
    for _ in 0..len1 { data.push(inner.clone()); }

    for i in 0..len1 {
        for j in 0..len2 {
            if i == 0 || j == 0 {
                if s1[i] == s2[j] {
                    data[i][j] = 1;
                }
            } else if s1[i] == s2[j] {
                data[i][j] = data[i - 1][j - 1] + 1;
            } else {
                data[i][j] = max(data[i - 1][j], data[i][j - 1]);
            }
        }
    }

    let mut i = len1 - 1;
    let mut j = len2 - 1;
    let mut r: Vec<usize> = Vec::with_capacity(len2);
    loop {
        if j == 0 || i == 0 { r.push(i); break; }
        if s1[i] == s2[j] {
            r.push(i);
            j -= 1;
            i -= 1;
        } else if data[i][j - 1] > data[i - 1][j] {
            j -= 1;
        } else {
            i -= 1;
        }
    }
    r.reverse();

    r
}