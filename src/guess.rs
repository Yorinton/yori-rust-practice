// structとnew関数で独自型
pub struct Guess {
    // privateなプロパティ
    value: u32,
}

impl Guess {
    pub fn new(num: u32) -> Self {
        if num > 100 {
            panic!("too big number: {}", num);
        } else if num < 1 {
            panic!("too small number: {}", num);
        }
        Guess {
            value: num
        }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}
