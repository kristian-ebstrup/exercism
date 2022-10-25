// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    students: Vec<String>,
    grades: Vec<u32>,
}

impl School {
    pub fn new() -> School {
        return School { students: Vec::new(), grades: Vec::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.push(String::from(student));
        self.grades.push(grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.grades.to_vec();
        grades.sort();
        grades.dedup();

        return grades;
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut vec: Vec<String> = Vec::with_capacity(self.grades.len());
        
        let iter = self.students.iter().zip(self.grades.iter());

        for student in iter {
            let (student_name, student_grade) = student;
            if student_grade == &grade {
                vec.push(String::from(student_name));
            }
        }

        vec.sort();

        return vec;
    }
}
