// -------------------------------------------------------------------
// ----------------------  Simple Get Request-------------------------
// -------------------------------------------------------------------

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error>{

//     let res = reqwest::get("https://www.rust-lang.org")
//     .await?;

//     let body = res
//     .text()
//     .await?;

//     println!("body = {:?}", body);
    
//     Ok(())
// }


// -------------------------------------------------------------------
// ----------------------  Simple Get Request-------------------------
// -------------------------------------------------------------------

// use serde_derive::{Serialize, Deserialize};

// #[derive(Deserialize)]
// struct Ip {
//     origin: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error>{
    
//     let ip = reqwest::get("http://httpbin.org/ip")
//     .await?
//     .json::<Ip>()
//     .await?;

//     println!("ip: {}", ip.origin);
//     Ok(())
// }

// -------------------------------------------------------------------
// ----------------------  Simple Post Request-------------------------
// -------------------------------------------------------------------

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error>{

//     let client = reqwest::Client::new();

//     let res = client.post("http://httpbin.org/post")
//         .body("the exact body that is sent")
//         .send()
//         .await?;

//     println!("Status code for response against request : {}",res.status());
    
//     Ok(())
// }


// -------------------------------------------------------------------
// ----------------- Simple Post Request For Forms -------------------
// -------------------------------------------------------------------


// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error>{
    
//     let params = [("foo", "bar"), ("baz", "quux")];
//     let client = reqwest::Client::new();
//     let res = client.post("http://httpbin.org/post")
//         .form(&params)
//         .send()
//         .await?;

//     println!("Status code for response against request : {}",res.status());
//     println!("HTTP version of response against request : {:?}",res.version());
//     println!("URL for response against request : {}",res.url());
//     println!("Length of content received in response : {:?}",res.content_length());
//     println!("Text of response : {:?}",res.text().await);
    
//     Ok(())
// }


// -------------------------------------------------------------------
// ----------------- Simple Post Request For JSON  -------------------
// -------------------------------------------------------------------

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let echo_json: serde_json::Value = reqwest::Client::new()
//         .post("https://jsonplaceholder.typicode.com/posts")
//         .json(&serde_json::json!({
//             "title": "Reqwest.rs",
//             "body": "https://docs.rs/reqwest",
//             "userId": 1
//         }))
//         .send()
//         .await?
//         .json()
//         .await?;

//     println!("{:#?}", echo_json);
//     // Object(
//     //     {
//     //         "body": String(
//     //             "https://docs.rs/reqwest"
//     //         ),
//     //         "id": Number(
//     //             101
//     //         ),
//     //         "title": String(
//     //             "Reqwest.rs"
//     //         ),
//     //         "userId": Number(
//     //             1
//     //         )
//     //     }
//     // )
//     Ok(())
// }

// -------------------------------------------------------------------
// --------------------- Blocking Get Request  -----------------------
// -------------------------------------------------------------------

fn main() -> Result<(), reqwest::Error>{

    let body = reqwest::blocking::get("https://www.rust-lang.org")?
        .text()?;

    println!("body = {:?}", body);
    
    Ok(())
}


// -------------------------------------------------------------------
// ----------------------  Blocking Post Request-------------------------
// -------------------------------------------------------------------
 
// fn main() -> Result<(), reqwest::Error>{
    
//     let client = reqwest::blocking::Client::new();
//     let res = client.post("http://httpbin.org/post")
//     .body("the exact body that is sent")
//     .send()?;

//     println!("Status code for response against request : {}",res.status());
//     println!("HTTP version of response against request : {:?}",res.version());
//     println!("URL for response against request : {}",res.url());
//     println!("Length of content received in response : {:?}",res.content_length());
//     println!("Text of response : {:?}",res.text());
    
//     Ok(())
// }