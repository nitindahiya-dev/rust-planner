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

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }

    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        let plan = prompt("Enter your plans: ");
        PlansInfo::new(plan)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed to convert to JSON")
    }

    pub fn write_to_file(&self) {
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

pub fn update_plan() -> Result<(), io::Error> {
    let plan_name = prompt("Enter your plan: ");
    let mut plans = read_plans_from_file()?;

    let mut found = false;

    for item in &mut plans {
        if item.plan == plan_name {
            found = true;
            item.plan = prompt("Enter your new plan: ");
        }
    }

    if found {
        let mut file = OpenOptions::new().write(true).truncate(true).open("plans.json")?;

        for item in &plans {
            let json_output = format!("{}\n", item.to_json());

            if let Err(e) = file.write_all(json_output.as_bytes()) {
                eprintln!("Error while writing plans: {}", e);
            }
        }
        println!("Plan '{}' updated successfully.", plan_name);
    } else {
        println!("Plan '{}' not found in the database.", plan_name);
    }

    Ok(())
}

pub fn delete_plan() -> Result<(), io::Error> {
    println!("Please give me the name of the plan you want to delete:");

    let plan_name = prompt("Enter your plan: ");

    let mut plans = read_plans_from_file()?;
    let mut found = false;

    plans.retain(|plan| {
        if plan.plan == plan_name {
            found = true;
            false
        } else {
            true
        }
    });

    if found {
        let mut file = OpenOptions::new().write(true).truncate(true).open("plans.json")?;

        for plan in &plans {
            let json_output = format!("{}\n", plan.to_json());
            if let Err(e) = file.write_all(json_output.as_bytes()) {
                eprintln!("Error while writing to file: {}", e);
            }
        }
        println!("Plan '{}' deleted successfully.", plan_name);
    } else {
        println!("Plan '{}' not found in the database.", plan_name);
    }

    Ok(())
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);  // Use `print!` instead of `println!` to avoid newline
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
