pub mod my_thread;

use log::{info};

/// # Main File for rust binary
/// everything start from main function


fn main() {
    println!("Hello, world!"); // basic way to print something to console
    info!("Log Test!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_helloworld(){
        assert!(true);
    }

    #[test]
    fn integer_clone(){
        let number:i32 = 1234;
        let mut another_number = number.clone();
        assert_eq!(another_number, 1234);
        another_number = 5678;
        assert_eq!(number, 1234); // integer clone doesn't affects origin
        assert_eq!(another_number, 5678);
    }

    #[test]
    fn str_clone(){
        let my_str = "hello world!";
        let mut new_str = my_str.clone();
        assert_eq!(new_str, "hello world!");
        new_str = "hello world again!";

        assert_eq!(my_str, "hello world!"); // &str clone doesn't affects origin
        assert_eq!(new_str, "hello world again!");
    }

    #[test]
    fn no_init_vars(){
        let a;
        a = 3;
        println!("{}",a);
        let my_string; // un-initialized variable doesn't need mutable keyword
        if a==3 {
            my_string = "A";
        } else {
            my_string = "B";
        }
    }

    #[test]
    fn loop_test(){
        let mut a = 0;
        while a<10 {
            println!("{}",a);
            a+=1;
        }
    }

    #[test]
    fn loop_test_2(){
        let mut a = 0;
        loop {
            println!("{}",a);
            a+=1;
            if a>10 {
                break;
            }
        }
    }

    #[test]
    fn create_query(){
        let start = 0;
        let end = 10;
        let size = 3;
        let mut res = Vec::new();
        let mut a = start;
        while a < end {
            println!("{}",a);
            res.push(format!("(Number={})",a.to_string()));
            a+=size;
        }
        println!("{:?}",res);
    }


    #[test]
    fn create_query_1(){
        let start = 0;
        let end = 10;
        let size = 3;
        let mut res = String::new();
        let mut a = start;
        loop{
            println!("{}",a);
            res.push_str(format!("(Number={})",a.to_string()).as_str());
            a+=size;
            if a > end {
                res.push(';');
                break;
            }
            res.push(',');
        }
        println!("{:?}",res);
    }
}