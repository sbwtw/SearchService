

#[derive(Debug)]
pub struct SearchContext {
    pub context: String,
}

impl SearchContext {
    pub fn new() -> Self {
        SearchContext {
            context: String::new(),
        }
    }

    pub fn search_all<T: AsRef<str>>(&self, pattern: T) -> Vec<(i32, i32)> {
        println!("Searching {} ...", pattern.as_ref());
        vec![(3, 5)]
    }
}