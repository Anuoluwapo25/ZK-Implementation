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
        self.phone as u32
    }

    // security: their shift is already in hours
    fn weekly_hours_in_building(&self) -> u32 {
        self.shift_hours
    }
}

pub fn print_usage<T: SchoolMember>(person: &T) {
    person.summary();
}

// Returns only the even numbers from a slice
pub fn vec_even(nums: &[u32]) -> Vec<u32> {
    nums.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

// Builds a HashMap from a vec of (key, value) pairs
pub fn vec_hash(pairs: Vec<(String, u32)>) -> HashMap<String, u32> {
    pairs.into_iter().collect()
}
