
mod my_thread {
    use std::thread;

    pub fn make_thread(){
        let handle = thread::spawn(||{
            println!("hello world!");
        });

        handle.join();
    }
}


#[cfg(test)]
mod tests {
    use super::my_thread;

    #[test]
    fn make_thread(){
        my_thread::make_thread();
    }

    #[test]
    fn make_many_thread(){
        for i in 1..10 {
            std::thread::spawn(move||{ // to use i in thread, add move keyword to spawn argument
                println!("Hello World! in thread {}", i);
            });
        }
    }
}