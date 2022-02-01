mod bfs;

use bitfield::bitfield;
use crate::Operator::{SubTwo, MulThree, AddFive};
use std::fs::{OpenOptions, File};
use std::io::{Write, Seek, SeekFrom, Read};

#[derive(Debug)]
enum Operator {
    SubTwo,
    MulThree,
    AddFive,
}

impl Operator {
    fn from_discriminant(d: u32) -> Self {
        match d {
            0 => SubTwo,
            1 => MulThree,
            2 => AddFive,
            _ => panic!("bad discriminant {}", d)
        }
    }

    fn discriminant(&self) -> u32 {
        match &self {
            SubTwo => {0}
            MulThree => {1}
            AddFive => {2}
        }
    }
}

bitfield! {
    struct Entry(u32);
    impl Debug;
    u32;
    previous, set_previous: 28, 0;
    _operator, _set_operator: 30, 29;
    _is_worked, _set_worked: 31, 31;
}

impl Entry {
    fn new(previous: u32, operator: Operator, is_worked: bool) -> Entry {
        let mut s = Self(0);

        s.set_previous(previous);
        s.set_operator(operator);
        s.set_worked(is_worked);

        s
    }


    fn value(&self) -> u32 {
        self.0
    }

    fn filled(&self) -> bool {
        self.0 != 0
    }

    fn operator(&self) -> Operator {
        Operator::from_discriminant(self._operator())
    }

    fn set_operator(& mut self, op: Operator) {
        self._set_operator(op.discriminant());
    }

    fn is_worked(&self) -> bool {
        self._is_worked() != 0
    }

    fn set_worked(& mut self, v: bool) {
        self._set_worked(if v { 1 } else { 0 });
    }
}

struct Map {
    _map: Vec<Entry>,
}

impl Map {
    fn new(size: u32) -> Self {
        let mut _map = Vec::with_capacity(size as usize);
        for _ in 0..size {
            _map.push(Entry(0));
        }
        //let _map = (0..size).map(|_| Entry(0)).collect();
        Self {
            _map,
        }
    }

    fn pass(& mut self, pass: usize) {

        let length = self._map.len() as usize;

        for i in 0..length {

            if i % (length / 100) == 0 {
                println!("Pass {}, {}%", pass, i / (length / 100));
            }

            let current = &self._map[i];

            if current.filled() && !current.is_worked() || i == 0 && !current.is_worked() {
                let value = i as u32;

                if value >= 2 {
                    let e = &mut self._map[value as usize - 2];
                    if !e.filled() {
                        *e = Entry::new(value, Operator::SubTwo, false);
                    }
                }

                if value * 3 < length as u32 {
                    let e = &mut self._map[value as usize * 3];
                    if !e.filled() && value != 0 {
                        *e = Entry::new(value, Operator::MulThree, false);
                    }
                }

                if value + 5 < length as u32 {
                    let e = &mut self._map[value as usize + 5];
                    if !e.filled() {
                        *e = Entry::new(value, Operator::AddFive, false);
                    }
                }

                self._map[i].set_worked(true);
            }
        }
    }

    fn filled(&self) -> u32 {
        let mut count = 0;
        for entry in self._map.iter() {
            if entry.filled() {
                count = count + 1;
            }
        }
        count
    }

    fn first_unfilled(&self) -> Option<u32> {
        for (i, entry) in self._map.iter().enumerate() {
            if !entry.filled() {
                return Some(i as u32);
            }
        }
        None
    }

    fn get(&self, n: u32) -> Option<Vec<Operator>> {

        if !self._map[n as usize].filled() {
            return None;
        }

        let mut x = n;

        let mut vec = Vec::new();

        while x != 0 {
            let entry = &self._map[x as usize];

            vec.insert(0,entry.operator());

            x = entry.previous();
        }

        Some(vec)
    }
}

fn calculate(ops: & Vec<Operator>) -> u32 {
    let mut x = 0;

    for op in ops {
        match op {
            SubTwo => {x = x - 2}
            MulThree => {x = x * 3}
            AddFive => { x = x + 5}
        }
    }

    x
}

fn get(file: & mut File, n: u32) -> Option<Vec<Operator>> {


    file.seek(SeekFrom::Start(n as u64 * 4 )).unwrap();
    let mut bytes = [0u8; 4];
    file.read_exact(& mut bytes[..]).unwrap();
    let entry = Entry(u32::from_be_bytes(bytes));
    if !entry.filled() {
        return None;
    }

    let mut x = n;

    let mut vec = Vec::new();

    while x != 0 {
        file.seek(SeekFrom::Start(x as u64 * 4)).unwrap();
        let mut bytes = [0u8; 4];
        file.read_exact(& mut bytes[..]).unwrap();
        let entry = Entry(u32::from_be_bytes(bytes));

        vec.insert(0,entry.operator());

        x = entry.previous();
    }

    Some(vec)
}

fn main() {
    println!("Hello, world!");

    let len = 536870912;

    //let mut m = Map::new(536870912);
    let mut m = Map::new(len);

    println!("allocated");

    for i in 0..3 {
        m.pass(i);
    }

    println!("first: {:?}", m.first_unfilled());

    println!("{}", m.filled());

    //show(& mut m);


    println!("get: {:?}", m.get(2357));

    let mut file = OpenOptions::new().create(true).write(true).truncate(true).read(true).open("./map/map").unwrap();

    for entry in m._map.iter() {
        let bytes = entry.0.to_be_bytes();

        file.write(&bytes[0..4]);
    }

    println!("{:?}", &get(& mut file, 1000000).unwrap());
    println!("{:?}", calculate(&get(& mut file, 1000000).unwrap()));

    //println!("test: {:?}", bfs::search(1000000));
}
