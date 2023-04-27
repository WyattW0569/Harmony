use rand::{thread_rng, Rng};

pub struct Testing {
    pub number: i32,
}

impl Testing {
    pub fn new() -> Testing {
        Testing {
            number: thread_rng().gen_range(0..100),
        }
    }
}
