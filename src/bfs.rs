use std::collections::HashMap;

fn unwind(map: & HashMap<i32, (i32, u8)>, n: i32) -> Vec<u8> {
    let mut v = n;

    let mut list = Vec::new();

    while v != 0 {
        let (next, op) = map.get(&v).unwrap();

        v = *next;
        list.push(*op);
    }

    list
}

fn round(map: & mut HashMap<i32, (i32, u8)>, n: i32) -> Option<()> {

    let map_clone = map.clone();

    for (&key, _) in map_clone.iter() {
        if !map_clone.contains_key(&(key - 2)) {
            map.insert(key - 2, (key, 2));
        }
        if !map_clone.contains_key(&(key * 3)) {
            map.insert(key * 3, (key, 3));
        }
        if !map_clone.contains_key(&(key + 5)) {
            map.insert(key + 5, (key, 5));
        }

        if key - 2 == n || key * 3 == n || key + 5 == n {
            return Some(())
        }
    }

    None
}

pub fn search(n: i32) -> Vec<u8> {
    let mut map = HashMap::new();

    map.insert(0, (0, 0));

    while let None = round(& mut map, n) { }

    println!("Map size: {}", map.len());

    unwind(& map, n)
}
