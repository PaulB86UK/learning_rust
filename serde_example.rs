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
    //implement display trait for the struct work so it can be easily used
    //whe using the println macro
    impl std::fmt::Display for Work {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "\n---\nWork:\n id: {}\n work_code: {}\n add_up_to: {}\n done: {}\n---",
                self.id, self.work_code, self.add_up_to, self.done,
            )
        }
    }
    //serialize the struct into a JSON String
    let json_str = serde_json::to_string(&work).unwrap();
    println!("Serialize: {}", json_str);

    //deserialize the struct into a JSON String
    let work_from_json_str: Work = serde_json::from_str(json_str.as_str()).unwrap();
    println!("DeSerialize: {:?}", work_from_json_str); //{:?} TODO Implement Display

    //check if the instances match
    assert_eq!(work, work_from_json_str);

    //print
    let work_from_json_str2: Work = serde_json::from_str(json_str.as_str()).unwrap();
    println!("DeSerialize v2: {}", work_from_json_str2); //Using Display this time
}
