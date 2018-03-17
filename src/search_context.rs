
use pinyin::*;

use std::cmp::{max, Eq, PartialEq};

// start, len
pub struct FuzzySearchResult {
    start: i32,
    len: i32,
}

impl FuzzySearchResult {
    pub fn to_string<T: AsRef<str>>(&self, context: T) -> String {
        let start = self.start as usize;
        let end = (self.start + self.len) as usize;

        context.as_ref()[start..end].to_string()
    }
}

impl Eq for FuzzySearchResult {}

impl PartialEq for FuzzySearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start &&
        self.len == other.len
    }
}


#[derive(Debug)]
pub struct SearchContext {
    context: String,
    context_chars: Vec<char>,
    pinyin_context: Vec<Vec<String>>,
    has_pinyin: bool,
}

impl SearchContext {
    pub fn new() -> Self {
        SearchContext {
            context: String::new(),
            context_chars: vec![],
            pinyin_context: vec![],
            has_pinyin: false,
        }
    }

    pub fn with_context<T: AsRef<str>>(context: T) -> Self {
        let mut r = SearchContext::new();
        r.set_context(context);

        r
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn set_context<T: AsRef<str>>(&mut self, context: T) {
        self.context = context.as_ref().to_string();
        self.context_chars = self.context.chars().collect();
        self.pinyin_context = pinyin(context.as_ref(), &Args::new());
        self.has_pinyin = self.pinyin_context.iter().any(|x| !x.is_empty());

        println!("{}\n{:?}", self.context, self.pinyin_context);
    }

    pub fn search<T: AsRef<str>>(&self, pattern: T) -> Option<usize> {
        self.context.find(pattern.as_ref())
    }

    pub fn search_all<T: AsRef<str>>(&self, pattern: T) -> Vec<i32> {
        let mut r = vec![];
        let mut i = 0;

        while let Some(idx) = self.context[i..].find(pattern.as_ref()) {
            r.push((idx + i) as i32);
            i += idx + 1;
        }

        r
    }

    pub fn fuzzy_search<T: AsRef<str>>(&self, pattern: T) -> Vec<usize> {
        let text: Vec<char> = pattern.as_ref().chars().collect();

        println!("{:?}", text);

        self.lcs_context(&text)
    }

    pub fn search_pinyin<T: AsRef<str>>(&self, pattern: T) -> (i32, i32) {

        (3, 5)
    }

    pub fn search_pinyin_all<T: AsRef<str>>(&self, pattern: T) -> Vec<(i32, i32)> {
        unimplemented!()
    }

    // longest common substring of context and text
    fn lcs_context(&self, text: &Vec<char>) -> Vec<usize> {
        let s1 = &self.context_chars;
        let s2 = text;
        let len1 = s1.len();
        let len2 = s2.len();

        let mut data: Vec<Vec<usize>> = Vec::with_capacity(len1);
        let mut inner = Vec::with_capacity(len2);
        for _ in 0..len2 { inner.push(0); }
        for _ in 0..len1 { data.push(inner.clone()); }

        let (mut max_i, mut max_j) = (0, 0);
        let mut max_num = 1;
        for i in 0..len1 {
            for j in 0..len2 {
                if i == 0 || j == 0 {
                    if s1[i] == s2[j] {
                        data[i][j] = 1;
                    }
                } else if s1[i] == s2[j] {
                    data[i][j] = data[i - 1][j - 1] + 1;

                    if data[i][j] > max_num {
                        max_num = data[i][j];
                        max_i = i;
                        max_j = j;
                    }
                } else {
                    data[i][j] = max(data[i - 1][j], data[i][j - 1]);
                }
            }
        }

        let mut i = max_i;
        let mut j = max_j;
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
}

#[cfg(test)]
mod test {

    use search_context::*;

    #[test]
    fn test_fuzzy_search_result() {
        let r = FuzzySearchResult { start: 2, len: 3, };

        assert_eq!(r.to_string("abcdef"), "cde".to_owned());
    }

    #[test]
    fn test_pinyin_is_empty() {
        let test = SearchContext::with_context("test1");
        assert_eq!(false, test.has_pinyin);

        let test = SearchContext::with_context("test1 测试1");
        assert_eq!(true, test.has_pinyin);
    }
}
