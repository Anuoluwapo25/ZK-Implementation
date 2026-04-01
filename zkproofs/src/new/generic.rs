use std::collections::HashMap;

pub trait SchoolMember {
    fn name(&self) -> &str;
    fn age(&self) -> i32;
    fn devices(&self) -> u32;
    fn weekly_hours_in_building(&self) -> u32;
    fn role(&self) -> &str;

    fn electricity_usage(&self) -> u32 {
        self.weekly_hours_in_building() * self.devices() * 10
    }

    fn summary(&self) {
        println!(
            "[{}] {} | age: {} | in building: {}hrs/week | devices: {} | electricity: {} units",
            self.role(),
            self.name(),
            self.age(),
            self.weekly_hours_in_building(),
            self.devices(),
            self.electricity_usage(),
        );
    }
}

pub struct Student {
    pub name: String,
    pub age: i32,
    pub laptop: bool,
    pub phone: bool,
    pub classes_per_week: u32,
}

pub struct Teacher {
    pub name: String,
    pub age: i32,
    pub laptop: bool,
    pub phone: bool,
    pub projector: bool,
    pub teaching_days_per_week: u32,
}

pub struct SecurityGuard {
    pub name: String,
    pub age: i32,
    pub phone: bool,
    pub shift_hours: u32,
}

impl SchoolMember for Student {
    fn name(&self) -> &str { &self.name }
    fn age(&self) -> i32 { self.age }
    fn role(&self) -> &str { "Student" }

    fn devices(&self) -> u32 {
        self.laptop as u32 + self.phone as u32
    }

    fn weekly_hours_in_building(&self) -> u32 {
        self.classes_per_week * 2
    }
}

impl SchoolMember for Teacher {
    fn name(&self) -> &str { &self.name }
    fn age(&self) -> i32 { self.age }
    fn role(&self) -> &str { "Teacher" }

    fn devices(&self) -> u32 {
        self.laptop as u32 + self.phone as u32 + self.projector as u32
    }

    fn weekly_hours_in_building(&self) -> u32 {
        self.teaching_days_per_week * 8
    }
}

impl SchoolMember for SecurityGuard {
    fn name(&self) -> &str { &self.name }
    fn age(&self) -> i32 { self.age }
    fn role(&self) -> &str { "Security" }

    fn devices(&self) -> u32 {
        self.phone as u32
    }

    fn weekly_hours_in_building(&self) -> u32 {
        self.shift_hours
    }
}

pub fn print_usage<T: SchoolMember>(person: &T) {
    person.summary();
}

pub fn vec_even(nums: &[u32]) -> Vec<u32> {
    nums.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

pub fn vec_hash(pairs: Vec<(String, u32)>) -> HashMap<String, u32> {
    pairs.into_iter().collect()
}

use std::collections::HashMap;
use std::collections::hash_map::Entry;

// =============================================
// Q1: word_frequencies (fixed + bonus)
// =============================================
fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut freq: HashMap<String, usize> = HashMap::new();

    for word in text.split_whitespace() {
        *freq.entry(word.to_string()).or_insert(0) += 1;
    }

    freq
}

fn print_duplicates(freq: &HashMap<String, usize>) {
    freq.iter()
        .filter(|(_, &count)| count > 1)
        .for_each(|(word, count)| {
            println!("{} appears {} times", word, count);
        });
}

fn main_q1() {
    let text = "hello world hello rust hello";
    let freq = word_frequencies(text);
    println!("Full frequencies: {:?}", freq);
    print_duplicates(&freq);
}

// =============================================
// Q2: Fix borrow checker (without cloning the value)
// =============================================
fn main_q2() {
    let mut scores: HashMap<&str, i32> = HashMap::new();

    scores.insert("Alice", 10);

   
    {
        let alice_score = scores.get("Alice");          
        println!("Alice's score: {:?}", alice_score);
    }

    scores.insert("Bob", 20);                           
    println!("Scores after Bob: {:?}", scores);
}

// =============================================
// Q3: Summable trait + impl for Vec<T>
// =============================================
trait Summable<T: std::ops::Add<Output = T> + Copy> {
    fn sum(self) -> T;
}

impl<T> Summable<T> for Vec<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    fn sum(self) -> T {
        let mut total = self[0];                     
        for &item in self.iter().skip(1) {
            total = total + item;                  
        }
        total
    }
}

fn main_q3() {
    let numbers: Vec<i32> = vec![10, 20, 30, 40];
    println!("Sum = {}", numbers.sum());           
}

// =============================================
// Q4: Generic Cache with get_or_insert_with
// =============================================
struct Cache<K, V> {
    store: HashMap<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn new() -> Self {
        Cache {
            store: HashMap::new(),
        }
    }

    fn get_or_insert_with<F>(&mut self, key: K, f: F) -> V
    where
        F: FnOnce() -> V,
    {
        match self.store.entry(key) {
            Entry::Occupied(entry) => entry.get().clone(), 
            Entry::Vacant(entry) => {
                let value = f();                        
                entry.insert(value.clone());           
                value
            }
        }
    }
}

fn main_q4() {
    let mut cache = Cache::new();
    let result1 = cache.get_or_insert_with("key1".to_string(), || {
        println!("Computing expensive value...");
        42
    });
    let result2 = cache.get_or_insert_with("key1".to_string(), || unreachable!()); // closure never runs
    println!("Result: {} {}", result1, result2);
}

// =============================================
// Q5 + Q6: longest with proper lifetimes
// =============================================
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main_q5_q6() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let result = longest(&s1, &s2);   
    println!("Longest: {}", result);

    println!("s1 is still alive: {}", s1);
}

// =============================================
// Q7: Three closures with correct Fn traits
// =============================================
fn main_q7() {
    // 1. FnOnce - consumes its argument
    let consume_string = |s: String| {
        println!("Consumed: {}", s);
    }; 

    consume_string("hello".to_string());

    // 2. FnMut - mutably borrows a vec
    let mut numbers = vec![1, 2, 3];
    let mut push_forty_two = |vec: &mut Vec<i32>| {
        vec.push(42);
    }; 

    push_forty_two(&mut numbers);
    println!("After push: {:?}", numbers);

    // 3. Fn - immutable borrow only
    let get_length = |s: &str| s.len();
    println!("Length of 'rust' = {}", get_length("rust"));
}

// =============================================
// Q8 + Q9: Macro + inner function vs closure
// =============================================
macro_rules! define_logger {
    ($struct_name:ident; $($field:ident: $ty:ty),*) => {
        #[derive(Debug)]
        struct $struct_name {
            $($field: $ty),*
        }

        impl $struct_name {
            fn log(&self) {
                fn print_field<T: std::fmt::Debug>(name: &str, value: &T) {
                    println!("  {} = {:?}", name, value);
                }

                println!("Logging {}:", stringify!($struct_name));
                $(print_field(stringify!($field), &self.$field);)*
            }
        }
    };
}

define_logger!(User; name: String, age: u8);

fn main_q8_q9() {
    let u = User {
        name: "Bob".into(),
        age: 30,
    };
    u.log();

    // Q9: inner function vs closure example
    fn outer() {
        fn inner() {
            println!("I am an inner named function");
        }
        inner();

        let captured = 42;
        let closure = || println!("Captured value: {}", captured);
        closure();
    }
    outer();
}


fn main() {
    main_q1();
    main_q2();
    main_q3();
    main_q4();
    main_q5_q6();
    main_q7();
    main_q8_q9();
}


// struct LruCache<K, V> {
//     pack: HashMap<K, V>,
//     order: VecDeque<K>
// }

// impl LruCache<K,V> {
//     fn new(capacity: usize) -> Self {
//         LruCache {
//             pack: HashMap::new(),
//             order: VecDeque::new()
//         }

//     }
//     fn get_or_compute<'a, F>(&mut self, key: K, f: F) -> V {


// }
// }


// fn word_frequencies(text: &str) -> HashMap<String, usize> {
//     let mut freq = HashMap::new();
//     for word in text.split_whitespace() {
//         *freq.entry(word.to_string()).or_insert(0).into_iter().filter(|(_, &count)| count > 1)
//     }
// }

// trait Summable {
//     fn sum(&self) -> T;

// }
// struct Add<T> {
//     a: T,
//     b: T,
// }
// impl Summable for Add {
//     fn sum(&self) -> T {
//         self.a + self.b
//     }
// }

// struct Cache(K, V) {
//     store: HashMap<K,V>,
// }

// impl Cache { 
//     fn new() -> Self {
//         Cache {
//             store: HashMap::new(),
//         }
//     }
//     fn get_or_insert_with(&mut self, key: K, f: impl FnOnce() -> V) -> V {
//         self.store.get_or_insert_with(key, || f())
//     }
// }
// use std::collections::HashMap;

// fn main() {
//     let mut scores: HashMap<&str, i32> = HashMap::new();
//     scores.insert("Alice", 10);
//     let alice_score = &scores.get("Alice"); // immutable borrow
//     scores.insert("Bob", 20);              // mutable borrow here
//     println!("{:?}", alice_score);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = String::from("world");
//     let result = longest_str(&s1, &s2); // your longest function
//     println!("{}", result);
//     // imagine more code that tries to use s1 or s2 here
// }

// fn longest(x: 'a string, y: 'a &str) -> &'a str { 
//    let longest = if x.len() > y.len() { x } else { y };
   
//    longest

// }

// fn nOnce(string: &str) -> String {
//     println({}, string);
// }

// fn mUt(vec: &mut Vec<i32>) -> i32 {
//     vec = Vec::new();
//     vec.push(42):
// }

// fn strinngs(str: &str) -> i32 {
//     str.len()
// }
// #[derive(Debug)]

// struct User {
//     name: String,
//     age: u8,
// }
// impl User {
//     fn log(&self) {
//         println("{}, {}", self.name, self.age);    }
// }

// define_logger!(User; name: String, age: u8);
// let u: User = User { name: "Bob".into(), age: 30 };

// u.log();