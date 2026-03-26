struct Countdown {
    n: i32,
}

impl Countdown {
    fn new(n: i32) -> Self {
        Countdown { n }
    }
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            let current_value = self.n;
            self.n = self.n - 1;
            return Some(current_value);
        }else{
            None
        }
    }
}

fn test(){
    let count = Countdown{
        n : 1
    }
}


