use std::{sync::{Arc, Mutex}, thread};

pub fn mutex(){
    let vec = Arc::new(Mutex::new(vec![1, 2, 3, 4]));
    let mut handlers = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&vec);
        //Spwaning a thread
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap();
            data.push(i);
            println!("Vec from thread is {}: {:?}", i, &data_clone);
        });

        handlers.push(handle);
    }
    // We have to connect it with the main thread as well.
    for handler in handlers{
        handler.join().unwrap();
    }

    println!("Vec from main is : {:?}", vec.lock().unwrap());
}