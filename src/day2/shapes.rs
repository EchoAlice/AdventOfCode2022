#[derive(Debug)]
pub struct Shape {
    pub name: String, 
    pub points: i32, 
    pub beats: String,
    pub ties: String,
    pub loses_to: String, 
}

// Can you instantiate a data structure without a function?
pub fn rock() -> Shape{
    let rock = Shape {
        name: String::from("rock"),
        points: 1,
        beats: String::from("scissors"), 
        ties: String::from("rock"),
        loses_to: String::from("paper"),
    };
    rock
}
pub fn paper() -> Shape {
    let paper = Shape {
        name: String::from("paper"),
        points: 2,
        beats: String::from("rock"), 
        ties: String::from("paper"),
        loses_to: String::from("scissors"),
    };
    paper
}
pub fn scissors() -> Shape{
    let scissors = Shape {
        name: String::from("scissors"),
        points: 3,
        beats: String::from("paper"), 
        ties: String::from("scissors"),
        loses_to: String::from("rock"),
    };
    scissors
}