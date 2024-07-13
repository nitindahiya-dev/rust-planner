mod lib;

use crate::lib::{
    delete_plan, prompt, read_plans_from_file, update_plan, PlansInfo,
};

fn clr() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let ascii = r#"


░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓███████▓▒░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░       ░▒▓██████▓▒░░▒▓█▓▒░      ░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░ 
░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░   ░▒▓█▓▒░    ░▒▓██████▓▒░       ░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░  ░▒▓█▓▒░      ░▒▓█▓▒░          ░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░  ░▒▓█▓▒░      ░▒▓█▓▒░          ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓███████▓▒░   ░▒▓█▓▒░      ░▒▓█▓▒░           ░▒▓██████▓▒░░▒▓████████▓▒░▒▓█▓▒░ 

"#;

    println!("{}", ascii);

    loop {
        println!("You can save your plans here");
        println!("1. Add Plan");
        println!("2. Edit Plan");
        println!("3. Delete Plan");
        println!("4. View All Plans");
        println!("5. Search Your Plans");
        println!("6. Exit now");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = PlansInfo::new(prompt("Plan: "));

                println!("Entry added Successfully");
                entry.write_to_file();
            }
            "2" => {
                clr();
                match update_plan() {
                    Ok(_) => println!("Plan successfully updated"),
                    Err(e) => eprintln!("Error While updating plan: {}", e),
                }
            }
            "3" => {
                clr();
                match delete_plan() {
                    Ok(_) => println!("Plan successfully deleted"),
                    Err(e) => eprintln!("Error while deleting plan: {}", e),
                }
            }
            "4" => {
                clr();
                let plans = read_plans_from_file().unwrap_or_else(|err| {
                    eprintln!("Error while reading plan: {}", err);
                    Vec::new()
                });

                for item in plans {
                    println!("Your plans in our database are: {}", item.plan)
                }
            }
            "5" => {
                clr();
                let plans = read_plans_from_file().unwrap_or_else(|err| {
                    eprintln!("Error searching plans: {}", err);
                    Vec::new()
                });

                let search = prompt("Search: ");
                let mut found = false;

                for item in &plans {
                    if item.plan == search {
                        println!("Your plan is: {}", item.plan);
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("Your plan is not in our database. Use option 1 to add plans.");
                }
            }
            "6" => {
                clr();
                println!("Feel Free to Try Again Later");
                break;
            }

            _ => println!("Please Use Option 1, 2, 3, 4, 5, or 6."),
        }
    }
}
