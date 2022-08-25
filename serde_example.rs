use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Work {
    pub id: i32,
    pub work_code: String,
    pub add_up_to: i32,
    pub done: bool,
}

fn main() {
    //Given a Rust Struct
    let work = Work {
        id: 0,
        work_code: "foo".to_string(),
        add_up_to: 100,
        done: false,
    };

    //serialize the struct into a JSON String
    let json_str = serde_json::to_string(&work).unwrap();
    println!("Serialize: {}", json_str);

    //deserialize the struct into a JSON String
    let work_from_json_str: Work = serde_json::from_str(json_str.as_str()).unwrap();
    println!("DeSerialize: {:?}", work_from_json_str);
}
