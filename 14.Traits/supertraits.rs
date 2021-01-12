trait Person {
  fn name(&self) -> String;
}

// Person is a supertrait of student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
  fn university(&self) -> String;
}

trait Programmer {
  fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer and Student.
// Implementing CompSciStudent requires to impl both supertraits.
trait CompSciStudent: Programmer + Student {
  fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
  format!(
    "My name is {} and I attend{}. My favourite language is {}. My Git username is {}",
    student.name(),
    student.university(),
    student.fav_language(),
    student.git_username(),
  )
}

fn main() {}