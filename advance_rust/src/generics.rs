trait Describable {
    fn describe(&self) -> String;
}
struct Item {
    name: String,
    price: i32,
}

impl Describable for Item {
    fn describe(&self) -> String {
        let strsd = format!("{} {} {} {}", {&self.name},":",{self.price},"cents");
        strsd
    }
}

fn main(){
    let name = Item{
        name : "Hello".to_string(),
        price : 10
    };
    name.describe();
}

#[derive(PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn are_equal(a: &Point, b: &Point) -> bool {
    (a.x == b.x) && (a.y == b.y)
}

fn distance_sq(a: &Point, b: &Point) -> i32 {
    let a_point = (a.x - b.x).pow(2);
    let b_point = (a.y - b.y).pow(2);
    a_point + b_point  
}

trait Summarize {
    type Output;
    fn summarize(&self) -> Self::Output;
}

struct Numbers { data: Vec<i32> }
struct Sentence { words: Vec<String> }

// Implement Summarize for Numbers (Output = i32, return sum)
impl Summarize for Numbers{
  type Output = i32;
  fn summarize(&self)-> Self::Output{
    self.data.iter().sum()
  }
}

// Implement Summarize for Sentence (Output = String, return joined words)
impl Summarize for Sentence{
    type Output = String;
  fn summarize(&self)-> Self::Output{
      self.words.join(" ")
  }
}