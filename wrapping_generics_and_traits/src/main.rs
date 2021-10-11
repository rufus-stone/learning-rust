/// Job trait
pub trait Job {
    fn salary(&self) -> u8;
}

/// Student
pub struct Student {
    job_title: String,
}

impl Job for Student {
    fn salary(&self) -> u8 {
        0
    }
}

/// Teacher
pub struct Teacher {
    job_title: String,
}

impl Job for Teacher {
    fn salary(&self) -> u8 {
        100
    }
}

/// Person - adding + ?Sized is necessary as otherwise the Vec<Box<Person<dyn Job>>> complains that "the size for values of type `dyn Job` cannot be known at compilation time"
// and that "the trait `std::marker::Sized` is not implemented for `dyn Job`"
pub struct Person<J: Job + ?Sized> {
    name: String,
    job: J,
}

fn main() {
    let student = Student {
        job_title: String::from("History Student"),
    };

    let teacher = Teacher {
        job_title: String::from("English Teacher"),
    };

    let alice: Person<Student> = Person {
        name: String::from("Alice"),
        job: student,
    };

    let bob: Person<Teacher> = Person {
        name: String::from("Bob"),
        job: teacher,
    };

    //let people = vec![Box::new(alice), Box::new(bob)]; // <-- This treats people as a Vec<Box<Person<Student>>>, so won't allow bob
    //let people = vec![Box::new(bob), Box::new(alice)]; // <-- This treats people as a Vec<Box<Person<Teacher>>>, so won't allow alice
    let people: Vec<Box<Person<dyn Job>>> = vec![Box::new(bob), Box::new(alice)]; // Without + ?Sized in the Person definition, this doesn't work!

    for person in people {
        println!("{} earns {:?}", person.name, person.job.salary());
    }
}
