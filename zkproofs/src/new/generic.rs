fn new() {
    mu_num = [1,2,3,4,5];
    your_ um =[3,5 ,6 ,7, 8];

    sum = mu_num[0]

    for i in 1..mu.len(){
        sum += mu_num[i];
    }
}

fn sum_of_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &i in arr {
        sum += i;
    }
    sum
}
trait SchoolMember {
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


struct Student {
    name: String,
    age: i32,
    laptop: bool,
    phone: bool,
    classes_per_week: u32, 
}

struct Teacher {
    name: String,
    age: i32,
    laptop: bool,
    phone: bool,
    projector: bool,
    teaching_days_per_week: u32, 
}

struct SecurityGuard {
    name: String,
    age: i32,
    phone: bool,
    shift_hours: u32, 
}


impl SchoolMember for Student {
    fn name(&self) -> &str { &self.name }
    fn age(&self) -> i32 { self.age }
    fn role(&self) -> &str { "Student" }

    fn devices(&self) -> u32 {
        self.laptop as u32 + self.phone as u32
    }

    // students: each class is 2 hours
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

    // teachers: full 8-hour days
    fn weekly_hours_in_building(&self) -> u32 {
        self.teaching_days_per_week * 8
    }
}

impl SchoolMember for SecurityGuard {
    fn name(&self) -> &str { &self.name }
    fn age(&self) -> i32 { self.age }
    fn role(&self) -> &str { "Security" }

    fn devices(&self) -> u32 {
        self.phone as u32 // just a phone
    }

    // security: their shift is already in hours
    fn weekly_hours_in_building(&self) -> u32 {
        self.shift_hours
    }
}

// --- Generic function using the trait ---

fn print_usage<T: SchoolMember>(person: &T) {
    person.summary();
}

fn main() {
    let student = Student {
        name: String::from("Amara"),
        age: 20,
        laptop: true,
        phone: true,
        classes_per_week: 10,
    };

    let teacher = Teacher {
        name: String::from("Mr. Obi"),
        age: 45,
        laptop: true,
        phone: true,
        projector: true,
        teaching_days_per_week: 5,
    };

    let guard = SecurityGuard {
        name: String::from("Emeka"),
        age: 35,
        phone: true,
        shift_hours: 56, // night shifts, 7 days
    };

    print_usage(&student);
    print_usage(&teacher);
    print_usage(&guard);
}
```

Output:
```
[Student] Amara | age: 20 | in building: 20hrs/week | devices: 2 | electricity: 200 units
[Teacher] Mr. Obi | age: 45 | in building: 40hrs/week | devices: 3 | electricity: 1200 units
[Security] Emeka | age: 35 | in building: 56hrs/week | devices: 1 | electricity: 560 units

fn vec_even(num: Vec<u32>) -> Vec<u32> {
    (0..num as u32).filter(|x| x % 2 == 0).collect()

    let mut even = Vec::new();

    // for i in 0..num.len() {
    //     if num[i] % 2 == 0 {
    //         even.push(num[i]);
    //     }
    // }
    // even

    for i in &num {
        if i % 2 == 0;
        even.push(*i)
    }
    even 
    }

    use std::collections::HashMap;

    fn vec_hash(tuple: Vec(<String>, <u32>)) -> Hashmaps<String, u32> {
        let mut map = HashMap::new();
        for (key,vaule)
    }

    fn main() {
        let even = vec_even([1,2,3,4,5]);
        println!("{:?}", even);
    }

    return even;

    fn even_vaules(num: Vec<u32>) -> Vec<u32> {
        let i = 0;

        while i < num.len() {
            for num[i] % 2 != 0 {
                num.remove(index: i);
            }
            i += 1;
        }
    }

