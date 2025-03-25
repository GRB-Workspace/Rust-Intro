use crate::garden::vegetables::Asparagus;

pub fn modules_intro() {
    let plant: Asparagus = Asparagus {
        name: String::from("Asparagus"),
        stalks: 10,
    };
    println!("{:?}", plant);
}

