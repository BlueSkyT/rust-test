use std::collections::HashMap;

fn print_array(v: &Vec<i32>) {
    for x in v {
        println!("{}", x);
    }
}

pub fn test_select() {
    let v = vec![1, 2, 3];
    print_array(&v);
    let mut ve = Vec::new();
    ve.push(122);
    ve.push(111);
    ve.push(23);
    print_array(&ve);
    // let ve1 = &mut ve;
    ve.pop();
    print_array(&ve);
}

pub fn test_hash_map() {
    let mut map = HashMap::new();
    map.insert(String::from("A"), 111);
    map.insert(String::from("B"), 2222);
    map.insert(String::from("C"), 3333);
    println!("{:?}", map);
    map.insert(String::from("D"), 777);
    println!("{:?}", map)
}

pub fn test_list() {
    let list = vec![1, 5, 3, 10, 2, 3];
    let result = largest(&list);
    println!("the largest is {}", result);

    let charts = vec!['a', 'd', 'c', 'a'];
    let charts_result = largest_char(&charts);
    println!("the largest char is {}", charts_result);
}

fn largest(list: &[i32]) -> &i32 {
    let mut first = &list[0];
    for item in list {
        if item > first {
            first = item;
        }
    }
    first
}

fn largest_char(list: &[char]) -> &char {
    let mut first = &list[0];
    for item in list {
        if item > first {
            first = item;
        }
    }
    first
}

