#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};

pub fn btree_map() {
    let mut inc_btree: BTreeMap<i32, i32> = BTreeMap::new();
    inc_btree.insert(1, 2);
    inc_btree.insert(3, 4);
    inc_btree.insert(2, 3);

    for (key, value) in inc_btree.iter() {
        println!("{key}: {value}")
    }

    println!("{inc_btree:?}")
}

pub fn hash_map() {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    hash_map.insert(1, 2);
    hash_map.insert(3, 4);
    hash_map.insert(2, 3);
    hash_map.insert(100, 101);

    for (key, value) in hash_map.iter() {
        println!("{key}: {value}")
    }

    println!(
        "Добавляем несуществующее значение {:?}, теперь по ключу 1000 мы получаем {:?}",
        hash_map.insert(1000, 1001),
        hash_map.get(&1000),
    );
    println!(
        "Добавляем существующее значение {:?}, теперь по ключу 100 мы получаем {:?}",
        hash_map.insert(100, 102),
        hash_map.get(&100),
    );

    println!("{hash_map:?}")
}
