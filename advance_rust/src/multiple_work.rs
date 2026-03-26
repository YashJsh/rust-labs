use rayon; // 1.11.0

pub fn multiple(){
    use rayon::prelude::*;
    
    let v1 = vec![1,2,3,4];
    let v2 = vec![21,12,131,12];
    let v3 = vec![123, 643, 23, 232];
    let v4 = vec![99,19, 11, 31];
    
    let v = vec![
        &v1,
        &v2,
        &v3, 
        &v4
    ];
    
    let sum : Vec<i32> = v
    .par_iter()
    .map(|&x| x.iter().sum::<i32>())
    .collect();

    println!("Sum is : {:?}", sum);
}