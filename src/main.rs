//https://www.crocus.co.kr/1147
//https://en.cppreference.com/w/cpp
//https://www.nextptr.com/
//dart언어랑 flutter프로그램
//dartpad.dartlang.org
//https://exercism.org/

// 0순위 c++
// rust https://play.rust-lang.org/
// 공부할거
// haxe 
// dart 
// nim
// go
// c#
// d
// zig
// odin
// v

// zig 컴파일러 https://mitchellh.com/zig

// https://dhghomon.github.io/easy_rust/Chapter_0.html
// https://www.youtube.com/watch?v=_VE1ueQj_tw
// https://doc.rust-lang.org/std/index.html
// https://exercism.org/tracks/rust/exercises/nth-prime/solutions/briankung


// https://rinthel.github.io/rust-lang-book-ko/ch16-01-threads.html



//fn main() {
//  let a = //String::from("string");
//  let b = a;
// rust언어의 특성 위의 상황이되면 a변수은 사용하지못함
// 이유는 b가 a의 주인이 되기때문에 b를 사용해야함
// 만약  let b = a.clone();이라 적으면 a를 복제한거기
// 때문에 a를 사용 할 수있음
// rust언어의 중요한 기능이므로 반드시 기억


  // println!("a : {}", a);
//  println!("b : {}", b);
//}

// 구조체 일때 사용가능한거
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//   let user1 = User {
//     email: String::from("someone@example.com"),
//     username: String::from("someusername123"),
//     active: true,
//     sign_in_count: 1,
//   };

//   let user2 = User {
//     email: String::from("another@example.com"),
//     ..user1
//   };
  
//   println!("email : {}", user1.email);
//   println!("email : {}", user2.email);
// }



// #[derive(Debug)]
// 위의것 열심히 사용하자
// struct Rectangle {
//   width: u32,
//   height: u32,
// }

// fn main() {
//   let rect1 = Rectangle {
//     width: 30,
//     height: 50,
//   };

//   println!("rect1 is {:?}", rect1);
//   println!("rect1 is {:#?}", rect1);
// }

// rev return string
// .chars().rev().collect::<String>()
// time 알아두자
// start + time::Duration::new(1000000000, 0)


// 참조
// #![allow(unused)]
// fn main() {
// #[derive(Debug,Copy,Clone)]
//   struct Point {
//   x: f64,
//   y: f64,
//   }
//   impl Point {
//     fn distance(&self,other:&Point) -> f64 {
//       let x_squared = f64::powi(other.x - self.x, 2); 
//       let y_squared = f64::powi(other.y - self.y, 2);
//       f64::sqrt(x_squared + y_squared)
//     }
// }
  
//   let p1 = Point { x: 0.0, y: 0.0 };
//   let p2 = Point { x: 5.0, y: 6.5 };
//   println!("p1 : {:?}", p1);
//   println!("p2 : {:?}", p2);
  
//   p1.distance(&p2);
//   (&p1).distance(&p2);
//   println!("p1.distance(&p2) : {:?}", p1);
  
//   println!("(&p1).distance(&p2) : {:?}", &p1);
// }

// fn main() {
// String
// .capacity
// .push
// .push_str
// .pop
// .with_capacity
// allocation
// let mut my_name = "".to_string();

//   let mut my_name = String::with_capacity(26);
//   println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());

//   my_name.push_str("David!");
//   println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());

//   my_name.push_str(" and I love in Seoul");
//   println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());

//   my_name.push('a');
//   println!("Length is {} and capacity is : {}", my_name.len(), my_name.capacity());

// }

// attribute
// const HS :i32 = 56;
// static SS :i32 = 0;
// 'static lifetime

// fn print_hs() {
//   println!("print_hs : {}", HS);
// }

// fn main() {
//   print_hs();
  
//   let my_name = "David"; // &'static str
// }

// ownership : 소유권

// let string가 return_it의 밖으로 못나감 오류 예제
// fn return_it() -> &String {
//   let string = String::from("대한민국");
//   &string // return
// }

// fn main() {
//   let return_string = return_it();
// }

// & immutable reference / shared reference
// &mut mutable reference / unique reference
// && **

// fn main() {
//   let mut my_num = 9;
//   // let num_ref = &mut my_num;
  
//   // *num_ref = 10;
//   // println!("my_num : {}", my_num);
  
//   let num_ref = &mut &mut my_num;
  
//   **num_ref = 10;
//   println!("my_num : {}", my_num);
    
// }

// fn main() {
//   let mut num = 10;
//   let num_ref = &num;
//   let num_change = &mut num;
//   *num_change += 10;
  
//   let mut num = 10;
//   let num_change = &mut num;
//   *num_change += 10;
//   let num_ref = &num;
//   println!("num_ref : {}", num_ref);
  
//  shadowing
  
//   let country = "대한민국";
//   let country_ref = &country;
//   let country = 8;
  
//   println!("country_ref : {}, country : {}", country_ref, country);
// }


// fn print_country1(country_name: String){
//   println!("country_name : {}", country_name);
// }

// fn print_country2(country_name: &String){
//   println!("country_name : {}", country_name);
// }

// fn main() {
//   let country_name = "대한민국".to_string();
//   print_country1(country_name);
//   print_country1(country_name);
  
//   print_country2(&country_name);
//   print_country2(&country_name);
// }

// fn add_is_great1(country_name: &mut String){
//     country_name.push_str(" is great");
//     println!("country_name : {}", country_name);
// }

// fn add_is_great2(mut country_name: String) { // take by value, declare as mutable
//   country_name.push_str(" is great");
//   println!("country_name : {}", country_name);
// }

// fn add_is_great3(mut country_name: String) -> String{ // take by value, declare as mutable
//   country_name.push_str(" is great");
//   println!("country_name : {}", country_name);
//   country_name
// }

// fn main() {
//   // let mut my_country = "캐나다".to_string();
//   // // add_is_great1(country_name); // by value
//   // add_is_great1(&mut my_country);
//   // add_is_great1(&mut my_country);
  
//   let my_country = "대한민국".to_string();
//   add_is_great2(my_country);
//   // add_is_great3(my_country);
  
// }


// 필요할듯
// fn main() {
//   let a = "hhh";
//   let b = "hhha";
//   let c = "hhhss";
  
//   println!("a : {0}, b : {1}, c : {2}", a, b, c);
//   println!("a : {1}, b : {2}, c : {0}", a, b, c);
//   println!("a : {x}, b : {y}, c : {z}", x = a, y = b, z = c);
//   println!("a : {z}, b : {x}, c : {y}", x = a, y = b, z = c);
// }

// It's trivial to copy the bytes

// ownership and copy types
// fn prints_number(number: i32) {
//   println!("my_num : {}", number);
// }

// fn prints_country(country: String) {
//   println!("my_country : {}", country);
// }


// // copy - copy types
// // clone - non-copy types
// fn main() {
//   let my_num = 8;
//   prints_number(my_num);
//   prints_number(my_num);
  
//   let my_country = "지원".to_string();
//   prints_country(my_country.clone());
//   prints_country(my_country);
// }


// fn main() {
//     let n = 10000;
// let mut con = 0;
//     let mut num = 3;
//     let mut decimal:Vec<u32> = vec![];
//     let mut loop_con = 0;
//     decimal.push(1);
//     decimal.push(2);
//     while con != n {
//         for i in 0..decimal.len() {
//             if num as u32 % decimal[i] as u32 == 0 {
//                 loop_con += 1;
//             }
//             if loop_con != 1 {
//                 break;
//             }
//         }
//         if loop_con == 1 {
//             decimal.push(num as u32);
//             con += 1;
//         }
//         loop_con = 0;
//         num += 1;
//     }
//     decimal
// }

// let mut con = 0;
// let mut num = 3;
// let mut decimal:Vec<u32> = vec![];
// let mut loop_con: bool = true;
// decimal.push(2);
// if n == 0 {
//     decimal[0]
// }else {
//     while con != n {
//         for i in 0..decimal.len() {
//             if num as u32 % decimal[i] as u32 != 0 {
//                 loop_con = true;
//             } else {
//                 loop_con = false;
//                 break;
//             }
//         }
//         if loop_con {
//             decimal.push(num as u32);
//             con += 1;
//         }
//         num += 1;
//     }
//     decimal[decimal.len() - 1]
// }


// 스택
// use std::io;

// fn main() {
//   let mut n = String::new();
//   let _ = io::stdin().read_line(&mut n).unwrap();
//   let n = n.trim().parse::<i32>().unwrap();

//   let mut v: Vec<i32> = Vec::new();
//   let mut input = String::new();

//   for _ in 0..n {
//     input.clear();
//     let _ = io::stdin().read_line(&mut input);
//     let mut splited = input.trim().split(' ');
//     let cmd = &splited.next().unwrap()[..2];

//     if cmd == "pu" {
//       v.push(splited.next().unwrap().parse::<i32>().unwrap());
//     } else if cmd == "to" {
//       if v.is_empty() {
//         println!("-1");
//       } else {
//         println!("{:?}", v.last().unwrap());
//       }
//     } else if cmd == "po" {
//       if v.is_empty() {
//         println!("-1");
//       } else {
//         println!("{:?}", v.pop().unwrap());
//       }
//     } else if cmd == "si" {
//       println!("{:?}", v.len());
//     } else if cmd == "em" {
//       println!("{:?}", if v.is_empty() { 1 } else { 0 });
//     } 
//   }
// }


// destructuring

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool
}

#[derive(Debug)]
struct Person2 {
    name: String,
    height: u8
}

impl Person2 {
    fn from_person(input: Person) -> Self {
        let Person {name, height, .. } = input;

        Self {
            name,
            height
        }
    }
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    
    let person2 = Person2::from_person(papa_doc);

    println!("Person2 type is : {:?}", person2);
}