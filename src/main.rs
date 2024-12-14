use std::fs::File;
use std::fs;
use std::path::Path;
use rand::Rng;
use std::io;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    first_name: String,
    last_name: String,
}
fn main() {


    let file_path = "team.json";
    match fs::metadata(file_path) {
        Ok(metadata) => {
            if metadata.is_file() {

                let json_file_path = Path::new("team.json");
                let file = File::open(json_file_path).expect("file not found");
                let users: Vec<User> = serde_json::from_reader(file).expect("error while reading or parsing");
            
                loop {
                println!("Please choose your volunteer.");
                let mut cmd = String::new();

                io::stdin()
                    .read_line(&mut cmd)
                    .expect("Failed to read line");
                // pick a volunteer
                let volunteer_number = rand::thread_rng().gen_range(0..users.len());

                let user = &users[volunteer_number];
                println!("Today's volunteer is: {} {} {}", user.first_name, user.last_name, volunteer_number)
                }
            } else {
                println!("Path exists, but it's not a json file.");
            }
        }
        Err(_) => {
            println!("File does not exist.");
        }
    };

}
