mod new;

use crate::new::generic::{Student, Teacher, SecurityGuard, print_usage, vec_even, vec_hash};

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
        shift_hours: 56,
    };

    print_usage(&student);
    print_usage(&teacher);
    print_usage(&guard);

    let even = vec_even(&[1, 2, 3, 4, 5]);
    println!("{:?}", even);

    let map = vec_hash(vec![
        (String::from("zk"), 10),
        (String::from("rust"), 8),
    ]);
    println!("{:?}", map);
}
