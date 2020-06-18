//------------------------------------------------------------
//                      Book Example 1
//------------------------------------------------------------ 

// use std::thread;
// use std::time::Duration;

// fn get_two_sites() {
//     // Spawn two threads to do work.
//     let thread_one = thread::spawn(||{         
//         // thread::sleep(Duration::from_millis(1000));
//         println!("Thread One")
//     });
//     let thread_two = thread::spawn(|| println!("Thread Two"));

//     // Wait for both threads to complete.
//     thread_two.join().expect("thread two panicked");
//     // thread_one.join().expect("thread one panicked");    
// }

// use futures::executor::block_on;
// use async_std::task;
// use futures::join;

// fn main(){
//     // get_two_sites();
//     block_on(get_two_sites_async());
// }

// async fn get_two_sites_async() {
//     // Create two different "futures" which, when run to completion,
//     // will asynchronously download the webpages.
//     let task_one = task::spawn(print_task1());
//     let task_two = task::spawn(print_task2());

//     // Run both futures to completion at the same time.
//     futures::join!(task_one, task_two);
// }

// async fn print_task1(){
//     println!("Task One");
// }

// async fn print_task2(){
//     println!("Task Two");
// }


//------------------------------------------------------------
//                      Book Example 2
//------------------------------------------------------------ 
//---------------------- Using await -------------------------

// use std::time::Duration;
// use futures::executor::block_on;
// use std::thread;
// use async_std::task;
// fn main(){

//     block_on(world());
// }

// async fn world(){
//     hello().await;
//     task::sleep(Duration::from_secs(5)).await; 
//     print!(" World");    
    
// }

// async fn hello(){      
//     print!("Hello");
          
// }

//------------------------------------------------------------
//                      Book Example 3
//------------------------------------------------------------ 


// use std::time::Duration;
// use futures::executor::block_on;
// use std::thread;
// fn main(){

//     hello();        
//     world();    
// }

// fn world(){
//     // thread::sleep(Duration::from_secs(7));
//     print!(" World");        
    
// }

// fn hello(){                
//     // thread::sleep(Duration::from_secs(2));  
//     print!("Hello");   
       
// }

//------------------------------------------------------------
//                      Book Example 4
//------------------------------------------------------------ 

// use futures::executor::block_on;
// use std::time::Duration;
// use std::thread;
// async fn learn_song() -> String { 
//     thread::sleep(Duration::from_secs(5));
//     "Songs".to_string() 
// }
// async fn sing_song(song: String) { println!("{}",song); }
// async fn dance() { println!("Dance"); }
// async fn learn_and_sing() {
//     // Wait until the song has been learned before singing it. We use `.await` here rather than `block_on` to prevent blocking the
//     // thread, which makes it possible to `dance` at the same time.
//     let song = learn_song().await;
//     sing_song(song).await;   
//     println!("Learn dancing and singing"); 
// }
// async fn async_main() {
//     let f1 = learn_and_sing();
//     let f2 = dance();
//     // `join!` is like `.await` but can wait for multiple futures concurrently. If we're temporarily blocked in the `learn_and_sing` future, the `dance`
//     // future will take over the current thread. If `dance` becomes blocked, `learn_and_sing` can take back over. If both futures are blocked, then
//     // `async_main` is blocked and will yield to the executor.
//     futures::join!(f1, f2);
// }
// fn main() {
//     block_on(async_main());
// }


//------------------------------------------------------------
//                      Book Example 5
//------------------------------------------------------------ 

// use futures::executor::block_on;

// async fn hello_world() {
//     println!("hello, world!");
// }

// fn main() {
//     let future = hello_world(); // Nothing is printed
//     block_on(hello_world()); // `future` is run and "hello, world!" is printed
// }

//------------------------------------------------------------
//                      Book Example 6
//------------------------------------------------------------ 


// use std::thread;
// use std::time::Duration;

// fn main() {

//     thread::spawn(move || {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(10));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(10));
//     }
// }


//------------------------------------------------------------
//                      Book Example 7
//------------------------------------------------------------ 

use futures::executor::block_on;
use futures::join;
use std::io;
use std::time::Duration;
use async_std::task;



fn main() -> io::Result<()> {

    println!("Loading start");

    // Block on the final future
    block_on(load_files());
    Ok(())
}

async fn load_files() {
    // Join the two futures together
    join!(load_file_1(), load_file_2());
}

async fn load_file_1() {
    // thread::sleep(Duration::from_secs(3));
    task::sleep(Duration::from_secs(5)).await;
    println!("Loading file 1");
}

async fn load_file_2() {
    // thread::sleep(Duration::from_secs(1));
    task::sleep(Duration::from_secs(2)).await;
    println!("Loading file 2");
}