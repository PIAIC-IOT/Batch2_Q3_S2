//------------------------------------------------------------
//                      Article Example 1
//------------------------------------------------------------ 

// use async_std::task;
// use futures::executor::block_on;
// use std::time::Duration;
// // ^ we need this for task spawning
// async fn negate_async(n: i32) -> i32 {
//     println!("Negating {}", n);
//     task::sleep(std::time::Duration::from_secs(5)).await;
//     println!("Finished sleeping for {}!", n);
//     n * -1
// }
// async fn f() -> i32 {
//     let neg = negate_async(1);
//     // ... nothing happens yet
//     let neg_task = task::spawn(negate_async(2));
//     // ^ this task /is/ started
//     task::sleep(std::time::Duration::from_secs(1)).await;
//     // we sleep for effect.
//     neg.await + neg_task.await
//     // ^ this starts the first task `neg`
//     // and waits for both tasks to finish
// }

// fn main() {
//     block_on(f());
// }

//------------------------------------------------------------
//                      Article Example 2
//------------------------------------------------------------ 

// use async_std::task;
// use surf;

// // fetch data from a url and return the results as a string.
// // if an error occurs, return the error.
// async fn fetch(url: &str) -> Result<String, surf::Exception> {
//     surf::get(url).recv_string().await
// }

// // execute the fetch function and print the results
// async fn execute() {
//     match fetch("https://pokeapi.co/api/v2/move/surf").await {
//         Ok(s) => println!("Fetched results: {:#?}", s),
//         Err(e) => println!("Got an error: {:?}", e),
//     };
// }

// fn main() {
//     task::block_on(execute());
//     // ^ start the future and wait for it to finish
// }


//------------------------------------------------------------
//                      Self Created Example
//------------------------------------------------------------ 

use surf;
use futures::future::try_join;
use async_std::task;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    task::block_on(async {
    let req1 = surf::get("https://en.wikipedia.org/wiki/Pakistan").recv_string();
    let req2 = surf::get("https://httpbin.org/get").recv_string();

    let (str1, str2) = try_join(req1, req2).await?;
    dbg!("{:?}",str2);
    dbg!("{:?}",str1);
    Ok(())
    })
}