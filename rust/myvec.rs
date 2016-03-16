use std::fmt::{self, Debug};
use std::iter::Iterator;

struct MyVec {
    data: Vec<i32>,
    length: u32,
}

enum Error {
    OutOfRange,
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of range!")
    }
}

impl MyVec {
    fn new() -> MyVec {
        MyVec {
            data: Vec::new(),
            length: 0,
        }
    }

    fn push_back(&mut self, val: i32) {
        let mut size = 0;
        // let mut it = self.data.into_iter();
        for _ in &self.data {
            size += 1;
        }
        let mut new_data = vec!{0; size + 1 as usize};
        let mut i = 0;
        while i < size {
            new_data[i] = self.data[i];
            i += 1;
        }
        new_data[i] = val;
        self.length += 1;

        self.data = new_data;
    }

    fn nth(&self, idx: usize) -> Result<&i32, Error> {
        match self.data.get(idx) {
            Some(val) => Ok(val),
            None => Err(Error::OutOfRange),
        }
    }

    fn iter(&self) -> MyVecIter {
        MyVecIter::new(&self)
    }
}

impl IntoIterator for MyVec {
    type Item = i32;
    type IntoIter = MyVecIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        MyVecIntoIter::new(self)
    }
}

struct MyVecIter<'a> {
    myvec: &'a MyVec,
    pos: usize,
}

impl<'a> MyVecIter<'a> {
    fn new(v: &'a MyVec) -> MyVecIter {
        MyVecIter {
            myvec: v,
            pos: 0,
        }
    }
}

impl<'a> Iterator for MyVecIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.myvec.nth(self.pos) {
            Ok(val) => {
                self.pos += 1;
                Some(val)
            },
            Err(_) => None,
        }
    }
}

struct MyVecIntoIter {
    myvec: MyVec,
    pos: usize,
}

impl MyVecIntoIter {
    fn new(v: MyVec) -> MyVecIntoIter {
        MyVecIntoIter {
            myvec: v,
            pos: 0,
        }
    }
}

impl Iterator for MyVecIntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.myvec.nth(self.pos) {
            Ok(val) => {
                self.pos += 1;
                Some(*val)
            },
            Err(_) => None,
        }
    }
}

fn main() {
    let mut v = MyVec::new();
    v.push_back(10);
    v.push_back(20);
    v.push_back(30);
    v.push_back(40);
    v.push_back(50);
    /*
    let mut i = 0;
    loop {
        match v.nth(i) {
            Ok(val) => {
                println!("[{}]: {}", &i, val);
                i += 1;
            },
            Err(err) => {
                println!("{:?}", err);
                break;
            },
        }
    }
    */
    println!("for with Iterator");
    for val in v.iter() {
        println!("{}", val);
    }

    println!("for with IntoIterator");
    for val in v {
        println!("{}", val);
    }
}
