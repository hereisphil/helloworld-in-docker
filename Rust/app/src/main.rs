// I was able to get the println!("Hello ASL!") working by myself
// With this video https://www.linkedin.com/learning/rust-essential-training/anatomy-of-a-rust-program?u=50813145
// With that, similar to Java, just run 'rustc main.rs' and then './main'
// However, for the datetime I watched a couple YT videos but then had
// ChatGPT explain and help me out with the whole Cargo thing
use chrono::Local;

fn main(){
    println!("Hello ASL!");

    let now = Local::now();
    println!("{}", now.format("%A, %B %d %Y • %H:%M:%S"));
}