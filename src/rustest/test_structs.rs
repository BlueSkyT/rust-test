use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
pub struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 22,
    }
}

pub fn test_struct() {
    let mut user1 = User {
        username: String::from("wuxh"),
        email: String::from("wxh@qq.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);
    user1.active = false;
    user1.sign_in_count = 999;
    println!("{:?}", user1);

    let new_user = build_user(String::from("wuxh@xxx.cn"), String::from("wuxh"));
    println!("{:?}", new_user);
}

pub fn test_object() {
    let name = "wuxh";
    let age = 20;
    let person = Person { name, age };
    println!("{:?}", person);
}

pub fn test_format() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", city)
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?}", *color);
    }
}