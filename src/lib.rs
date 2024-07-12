use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlansInfo {
    pub plan: String,
}

impl PlansInfo {
    pub fn new(plan: String) -> Self {
        PlansInfo { plan }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed to convert to JSON")
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }

    pub fn write_to_json(&self) {
        let json_output = format!("{}\n", self.to_json());

        match OpenOptions::new().create(true).append(true).open("plans.json") {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Failed to write to file: {}", e);
                } else {
                    println!("Successfully wrote to ./plans.json");
                }
            }
            Err(e) => eprintln!("Error opening file: {}", e),
        }
    }
}

pub fn read_plans_from_file() -> Result<Vec<PlansInfo>, io::Error> {
    let file = File::open("plans.json")?;
    let reader = io::BufReader::new(file);
    let mut plans: Vec<PlansInfo> = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(plan_info) = PlansInfo::from_json(&json_string) {
                plans.push(plan_info);
            }
        }
    }
    Ok(plans)
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);  // Use `print!` instead of `println!` to avoid newline
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
