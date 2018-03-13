
use pinyin::*;

#[derive(Debug)]
pub struct SearchContext {
    context: String,
    pinyin_context: Vec<Vec<String>>,
}

impl SearchContext {
    pub fn new() -> Self {
        SearchContext {
            context: String::new(),
            pinyin_context: vec![],
        }
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn set_context<T: AsRef<str>>(&mut self, context: T) {
        self.context = context.as_ref().to_string();
        self.pinyin_context = pinyin(context.as_ref(), &Args::new());
    }

    pub fn search<T: AsRef<str>>(&self, pattern: T) -> i32 {
        match self.context.find(pattern.as_ref()) {
            Some(idx) => idx as i32,
            None => -1,
        }
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

    pub fn search_pinyin<T: AsRef<str>>(&self, pattern: T) -> (i32, i32) {

        (3, 5)
    }

    pub fn search_pinyin_all<T: AsRef<str>>(&self, pattern: T) -> Vec<(i32, i32)> {
        unimplemented!()
    }
}