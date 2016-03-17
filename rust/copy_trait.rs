struct Num {
    int: i32,
}

impl Num {
    fn new(i: i32) -> Num {
        Num {
            int: i,
        }
    }
}

impl Default for Num {
    fn default() -> Num {
        Num {
            int: 0,
        }
    }
}

impl Copy for Num {}
impl Clone for Num {
    fn clone(&self) -> Num {
        *self
    }
}

fn main() {
    let a: [Num; 10] = [Default::default(); 10];
}
