use bson::{doc, Bson};
use mongodb::{options::ClientOptions, Client};

extern crate serde_json;

fn main() -> mongodb::error::Result<()> {
    // Parse a connection string into an options struct.
   
    // ----------- Conecting to local mongoDB server --------------
    // let mut client_options = ClientOptions::parse("mongodb://localhost:27017/")?;mongodb+srv://readUserMongo:OrhUQPE6f2BsYHmZ@cluster0-bhwq5.mongodb.net/example?retryWrites=true&w=majority
    
    // ----------- Conecting to mongoDB Atlas cluster -------------
    let mut client_options = ClientOptions::parse("mongodb+srv://readUserMongo:OrhUQPE6f2BsYHmZ@cluster0-bhwq5.mongodb.net/example?retryWrites=true&w=majority")?;
    
    // Manually set an option.
    client_options.app_name = Some("Rust demo".to_string());

    // Get a handle to the deployment.
    println!("--------- DB connection test 1 --------");
    let client = Client::with_options(client_options)?;
    
    // ------ Uncomment below lines to test the server using ping ------
    // client
    //     .database("example")
    //     .run_command(doc! {"ping": 1}, None)?;
    // println!("Connected successfully.\n");

    // List the names of the databases in that deployment.
    println!("--------- DB connection test 2 --------");
    for db_name in client.list_database_names(None)? {
        println!("{}", db_name);
    }

    /** ---------------------------------------------------------------
     *      After connecting to DB now manipulate with collections 
     * ----------------------------------------------------------------*/

    // Get a handle to a database.
    let db = client.database("example");

    // List the names of the collections in that database.
    println!("\n--------- Listing collections in specified DB --------");
    for collection_name in db.list_collection_names(None)? {
        println!("{}", collection_name);
    }

    /** ---------------------------------------------------------------
     *                Inserting data into collections
     *  --------------------------------------------------------------- */
    
     // Get a handle to a collection in the database.
    let collection = db.collection("inventory");
    
    // Uncomment below lines to insert single record
    // println!("\n--------- Inserting single record --------");
    // let doc = doc! { "item": "Diary"};
    // let record = collection.insert_one(doc, None);
    // println!("Newly inserted record : {:?}", record);

    // Uncomment below lines to insert multiple records
    // let docs = vec![
    //     doc! { "_id": 00000008 , "item": "mask1", "qty": 23, "status": "B", "size": { "h": 10, "w": 33, "uom": "mm"} },
    //     doc! { "_id": 00000009 , "item": "key1", "qty": 23, "status": "B", "size": { "h": 10, "w": 33, "uom": "mm"} },
    //     doc! { "_id": 00000010 , "item": "hanger1", "qty": 23, "status": "B", "tags": ["Blue", "Green"] },
    // ];

    // Insert some documents into the "example.inventory" collection.
    // println!("\n--------- Inserting multiple records --------");
    // collection.insert_many(docs, None)?;

    /** --------------------------------------------------------------
     *                 replacing record in collections 
     *  --------------------------------------------------------------*/
    
    // uncomment below lines to replace an existing record completey with a new one
    //  println!("\n--------- Replacing single record --------");
    // let filter = doc! { "item": "hanger1" };
    // let replacement = doc! { "qty": 123 };
    // collection.find_one_and_replace(filter, replacement, None);

    /** -------------------------------------------------------------
     *                 updating record in collections
     *  ------------------------------------------------------------- */

    //  update inventory WHERE item = "mobile" SET qty = 123;
    // println!("\n--------- Updating single record 1 --------");
    // let filter = doc! { "item": "mobile" };
    // let update = doc!{"$set" => { "qty" => 123 } };
    // collection.find_one_and_update(filter, update, None);

    // println!("\n--------- Updating single record 2 --------");
    // let filter = doc! { "item": "mobile" };
    // let update = doc!{"$set" => { "qty" => 333 } };
    // collection.update_one(filter, update, None);

    // println!("\n--------- Updating multiple records --------");
    // let filter = doc! { "item": "mobile" };
    // let update = doc! {"$set" => { "item" => "Iron" } };
    // collection.update_many(filter, update, None);


    /** -------------------------------------------------------------
     *                 Deleting record in collections
     *  ------------------------------------------------------------- */

    // println!("\n--------- Deleting single record --------");
    // let filter = doc! { "item" : "Diary" };
    // collection.delete_one(filter, None);

    // println!("\n--------- Deleting multiple records --------");
    // let filter = doc! { "item" : "laptop" };
    // collection.delete_many(filter, None);


    /** ------------------------------------------------------------ 
     *                    Querying collections
     *  ------------------------------------------------------------ */   
    let mut cursor = collection.find(None, None)?;

    // Iterate over the results of the cursor.
    // println!("\n--------- Listing records in specified collection --------");
    // while let Some(result) = cursor.next() {
    //     match result {
    //         Ok(document) => {
    //             if let Some(item) = document.get("item").and_then(Bson::as_str) {
    //                 println!("Item name: {:#?}", document);
    //             }  else {
    //                 println!("no item found");
    //             }
    //         }
    //         Err(e) => return Err(e.into()),
    //     }
    // }

    let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    let json_data = serde_json::to_string(&docs).unwrap();
    println!("{:?}", json_data);

    // for ()

    Ok(())
}