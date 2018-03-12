
use pinyin::*;

#[derive(Debug)]
pub struct SearchContext {
    pub context: String,
    pinyin_context: Option<String>,
}

impl SearchContext {
    pub fn new() -> Self {
        SearchContext {
            context: String::new(),
            pinyin_context: None,
        }
    }

    pub fn search<T: AsRef<str>>(&self, pattern: T) -> (i32, i32) {
        unimplemented!()
    }

    pub fn search_all<T: AsRef<str>>(&self, pattern: T) -> Vec<(i32, i32)> {
        println!("Searching {} ...", pattern.as_ref());
        vec![(3, 5)]
    }

    pub fn search_pinyin<T: AsRef<str>>(&mut self, pattern: T) -> (i32, i32) {

        println!("{:?}", self.pinyin_context());

        (3, 5)
    }

    pub fn search_pinyin_all<T: AsRef<str>>(&self, pattern: T) -> Vec<(i32, i32)> {
        unimplemented!()
    }

    fn pinyin_context(&mut self) -> &str {
        let ref c = self.context;

        self.pinyin_context.get_or_insert_with(|| {
            let s = pinyin(&c, &Args::new());
            println!("{:?}", s);
            "asd".to_owned()
        })
    }
}