use std::collections::HashSet;

use rayon::vec;

mod generics;
mod multiple_work;
mod mutex;
mod refcell;
mod iterator;

fn main() {
    mutex::mutex();
    refcell::refcell();
    multiple_work::multiple();
    // transform("sdsdfa");
    // let input = "  42  ";
    // let output = transform(input);
    // println!("{}", output);
    // let mut arr = vec![1,2,3,4];
    // double_all(&mut arr);
    // let value = coin_value("penny");
    // println!("{}", value);
}

fn sum_of_squares(nums: &[i32]) -> i32 {
    let squares : Vec<i32>= nums.iter()
    .map(|n|  n * n)
    .collect();
    squares.iter().sum()
}

fn evens_only(nums: &[i32]) -> Vec<i32> {
    let ans = nums.iter()
    .filter(|&&n| n%2==0)
    .copied()
    .collect();
    ans
}



fn double_all(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .map(|n| n * 2) // This returns the new value
        .collect()
}

fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    if s.starts_with(prefix) {
        &s[prefix.len()..]
    } else {
        s
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for i in 1..list.len() {
        if list[i] > largest {
            largest = list[i]
        }
    }
    largest
}

fn parse_csv_sum(s: &str) -> Result<i32, String> {
    if s.is_empty() {
        return Err("empty input".to_string());
    }
    let mut numbers: Vec<i32> = s
        .split(",")
        .map(|n| {
            n.parse::<i32>()
                .map_err(|_| format!("invalid number: {}", n))
        })
        .collect::<Result<Vec<i32>, String>>()?;
    let sm: i32 = numbers.iter().sum();
    println!("{:?}", numbers);
    Ok(sm)
}

fn parse_number(s: &str) -> Result<i32, String> {
    let parsed_str = s.parse();

    match parsed_str {
        Ok(n) => Ok(n),
        Err(_) => Err("invalid number".to_string()),
    }
}

fn add_parsed(a: &str, b: &str) -> Result<i32, String> {
    // Parse both strings and return their sum
    // Use ? to propagate errors
    let a_parse: i32 = a.parse().map_err(|_| "invalid number".to_string())?;
    let b_parse: i32 = b.parse().map_err(|_| "invalid number".to_string())?;
    Ok(a_parse + b_parse)
}

fn first_word(s: &str) -> String {
    let mut words = s.split_whitespace();

    if let Some(first) = words.next() {
        return first.to_string();
    } else {
        return s.to_string();
    }
}

fn transform(s: &str) -> i32 {
    // Shadow a variable through each step
    let trimmed_str = s.trim().len();
    println!("{}", trimmed_str);
    trimmed_str as i32
}



struct Rectangle {
    // Define width and height fields
    height: i32,
    weight: i32,
}

// impl Rectangle {
//     fn square(size: i32) -> Self {
//         Self{
//           side : size
//         }
//     }

//     fn area(&self) -> i32 {
//         self.height * self.weight
//     }
// }

fn coin_value(coin: &str) -> i32 {
    // Use match on the coin string
    match coin {
        "peeny" => return 1,
        "nickel" => return 5,
        "dime" => return 10,
        "quarted" => return 25,
        _ => return 0,
    }
}

fn count_vowels(s: &str) -> usize {
    let c: Vec<char> = s.chars().collect();
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut total = 0;
    for i in 0..c.len() {
        if vowels.contains(&c[i]) {
            total += 1;
        }
    }
    total
}
