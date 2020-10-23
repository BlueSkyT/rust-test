#[test]
fn f_test() {
    assert_eq!(4, 5)
}

#[test]
fn f_test2() {
    assert_eq!(4, 4)
}

#[test]
fn test_println() {
    println!("{} days", 31)
}

#[test]
fn test_data_type() {
    //test parse int
    let i: u32 = "42".parse().expect("please try a number!");
    assert_eq!(i, 42)
}

#[test]
fn test_reference() {
    let mut s = String::from("hello");
    s.push_str(", world");
    assert_eq!("hello, world", s)
}

#[test]
fn test_slice() {
    let s = [1, 2, 3, 4];
    for i in s.iter() {
        println!("{}", i)
    }

    println!("{}", s.len());
    let s1 = &s[0..2];
    for i in s1.iter() {
        println!("{}",i)
    }
    println!("test string");
    let a = String::from("abcdef");
    let as1 = &a[..];
    let as2 = &a[1..3];
    println!("as1:{}, as2:{}", as1, as2)
}