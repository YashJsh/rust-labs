use std::{collections::HashMap, cell::RefCell};

fn expensive_calculation()-> String{
    let calculation = String::from("Expensive String");
    return calculation;
}

struct ExpensiveData{
    data :RefCell<HashMap<u32, String>>
}

impl ExpensiveData{
    fn new()-> Self{
        ExpensiveData{
            data : HashMap::new().into(),
        }
    }
    fn get_value(&mut self, input : u32) -> String{
        let mut data = self.data.borrow_mut();
        if !data.contains_key(&input){
            let value = expensive_calculation();
            data.insert(input, value);
        }
        data.get(&input).unwrap().clone()
    }
}

pub fn refcell(){
    let mut cache = ExpensiveData::new();
    let v1 = cache.get_value(1);
    let v2 = cache.get_value(1);
    let v3 = cache.get_value(1);
    
    println!("v1: {}, v2 : {}, v3 : {}", v1, v2, v3);
}