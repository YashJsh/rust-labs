use std::collections::HashMap;

pub fn most_frequent(s: &str){
    let mut map : HashMap<String, i32> = HashMap::new();
    let input : Vec<String> = s.split(" ").map(|c| c.to_lowercase()).collect();
    println!("{:?}", input);
    for word in input.iter() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    println!("{:?}", map);
    let mut highest = 0;
    let mut result: Option<String> = None;
    for (i, j) in map.iter(){
        if j > &highest{
            highest = *j;
            result = Some(i.clone());
        }
    }
    println!("{}", result.unwrap());
}

