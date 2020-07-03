// -------------- Example 1.1-a ---------------------
// ---------------------------------------------------
// Execution of multiple threads delayed/non-delayed

// use std::thread;
// use std::time::Duration;

// fn get_two_sites() {
//     // Spawn two threads to do work.

//     let thread_one = thread::spawn(||{         
//         thread::sleep(Duration::from_millis(2000));
//         println!("Thread One")
//     });

//     let thread_two = thread::spawn(||{
//         println!("Thread Two")
//     });

//     // Wait for both threads to complete.
//     thread_one.join().expect("thread one panicked");  
//     thread_two.join().expect("thread two panicked");  
// }

// fn main(){
//     get_two_sites();
// }


// -------------- Example 1.1-b ---------------------
// ---------------------------------------------------
// Execution of multiple async fn delayed/non-delayed

// use std::time::Duration;
// use std::thread;
// use futures::executor::block_on;
// // use futures::future::join;

// async fn download_async1(text : &str) {
//     thread::sleep(Duration::from_millis(4000)); //Uncomment
//     println!("{:?}",text);
// }

// async fn download_async2(text : &str) {
//     println!("{:?}",text);
// }

// async fn get_two_sites_async() {

//     let future_one = download_async1("https://www.foo.com");
//     let future_two = download_async2("https://www.bar.com");

//     futures::join!(future_two, future_one);  
// }

// fn main(){
//     block_on(get_two_sites_async());
//     println!("Hello world!")
// }


// -------------- Example 1.3-a1 ---------------------
// ---------------------------------------------------
// Using "executor::block_on()"

// use futures::executor::block_on;

// //An asynchronous function which will return a "future (a task)"
// async fn hello_world() {
//     println!("hello, world!");
// }

// fn main() {
//     let future = hello_world(); // Nothing is printed
//     block_on(future); // `future` is run and "hello, world!" is printed
//     println!("Hello IoT");
//     // future.await;
// }


// -------------- Example 1.3-a2 ---------------------
// ---------------------------------------------------
// Using "await"

// use futures::executor::block_on;

// //An asynchronous function which will return a "future (a task)"
// async fn hello_world() {
//     println!("hello, world!");
// }

// async fn another_function() {
//     hello_world().await;
// }

// fn main() {
//     block_on(another_function());
// }


// -------------- Example 1.3-b1 ---------------------
// ---------------------------------------------------
// Multiple async fn using block_on non delayed

// use futures::executor::block_on;

// async fn get_sum() -> f32 {
//     65.2
// }

// async fn calculate_grade(sum : f32) {

//     if sum > 50.0 {
//         println!("Candidate is passed");
//     } else {
//         println!("Candidate is failed");
//     }
// }

// async fn print_sum() {
//     println!("Sum is {}",65.4);
// }

// fn main() {
//     let sum = block_on(get_sum());
//     block_on(calculate_grade(sum));
//     block_on(print_sum());
// }



// -------------- Example 1.3-b2 ---------------------
// ---------------------------------------------------
// Multiple async fn using block_on delayed for making difference

use std::thread;
use futures::executor::block_on;
use std::time::Duration;

async fn get_sum() -> f32 {
    65.2
}

async fn calculate_grade(sum : f32) {

    thread::sleep(Duration::from_secs(2));
    if sum > 50.0 {
        println!("Candidate is passed");
    } else {
        println!("Candidate is failed");
    }
}

async fn print_sum() {
    println!("Sum is {}",54.2);
}

fn main() {
    let sum = block_on(get_sum());
    block_on(calculate_grade(sum));
    block_on(print_sum());
}


// -------------- Example 1.3-b3 ---------------------
// ---------------------------------------------------
// Multiple async fn using .await delayed for making difference


// use std::thread;
// use futures::executor::block_on;
// use std::time::Duration;

// async fn get_sum() -> f32 {

//     65.2
// }

// async fn calculate_grade(sum : f32) {

//     thread::sleep(Duration::from_secs(3));
//     if sum > 50.0 {
//         println!("Candidate is passed");
//     } else {
//         println!("Candidate is failed");
//     }
// }

// async fn print_sum(sum : f32) {
    
//     println!("Sum is {}",sum);
// }

// async fn get_sum_and_calculate_grade() {
    
//     let sum = get_sum().await;
//     calculate_grade(sum).await;
// }

// async fn oscillator() {
    
//     let f1 = get_sum_and_calculate_grade();
//     let f2 = print_sum(50.62);

//     futures::join!(f2, f1);
// }

// fn main() {
//     task::block_on(oscillator());
// }
