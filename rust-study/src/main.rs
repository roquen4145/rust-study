pub mod my_thread;

/// # Main File for rust binary
/// everything start from main function


fn main() {
    println!("Hello, world!"); // basic way to print something to console
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
}