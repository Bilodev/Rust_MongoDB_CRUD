
use tokio;
pub use mongodb::{bson::doc,bson::Document,options::{ClientOptions, CollectionOptions}, Client};

#[warn(non_snake_case)]
mod Obj;

#[tokio::main]
#[warn(unused)]
pub async fn main() -> mongodb::error::Result<()> {
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb+srv://antonio:antonio@cluster0.mk39l.mongodb.net/myFirstDatabase?retryWrites=true&w=majority")
            .await?;
    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    println!("Connected successfully.");


    let db = client.database("Rust");
    let collection = db.collection::<Document>("user");
    // List the names of the databases in that cluster
    

    //Start Code
    use terminal_menu::{menu, button, run, mut_menu};
    'main: loop{
        
        let menu = menu(vec![
            button("Create"),
            button("Read"),
            button("Update"),
            button("Delete"),
            button("Exit"),
        ]);
        println!("");
        run(&menu);
        println!("");


    
        let mut username = String::new();
        let mut email= String::new();
        let mut phone= String::new();

        // you can get the selected buttons name like so:
        if mut_menu(&menu).selected_item_name() == "Create"{
            println!("Insert Name");
            username = Obj::readln(username);

            println!("Insert Email");
            email = Obj::readln(email);

            println!("Insert Phone Number");
            phone = Obj::readln(phone);
            
            let username = username.trim();
            let email = email.trim();
            
            let phone:u32 = phone.trim().parse().expect("Error Converting");
              
            let u = doc! {"Username":username,"Email":email, "Phone":phone};
            collection.insert_one(u, None).await?;
            println!("Customer Added Succesfully"); 
        }


        else if  mut_menu(&menu).selected_item_name() == "Read"{
            println!("Insert Email of the customer");
            let email = Obj::readln(email);
            let email = email.trim();

            let result = collection.find_one(
                doc! {
                   "Email": email
                },
                None,
             ).await?;

             println!("{:#?}",result);
        }


        else if mut_menu(&menu).selected_item_name() == "Update"{
            let mut email:String = String::new();


            println!("Insert Email of the old user");
            let mut email = Obj::readln(email);

            let mut email = email.trim();

    

            println!("Insert new Email");
            let mut newemail = String::new();
            newemail = Obj::readln(newemail);
            let newemail = newemail.trim();

            
            let update_result = collection.update_one(
                doc! {
                     "Email": email
                },
                doc! {
                   "$set": { "Email": newemail }
                },
                None,
             ).await?;
             println!("Updated {} document", update_result.modified_count);
        }

        
        else if  mut_menu(&menu).selected_item_name() == "Delete"{
            println!("Insert Email of the old user");
            let email = Obj::readln(email);
            let email = email.trim();
        

            let delete_result = collection.delete_one(
                doc! {
                   "Email": email
                },
                None,
             ).await?;
             println!("Updated {} document", delete_result.deleted_count);     
        }
    
        
        else if mut_menu(&menu).selected_item_name() == "Exit"{
            break 'main;
        } 

    }

    Ok(())
}

