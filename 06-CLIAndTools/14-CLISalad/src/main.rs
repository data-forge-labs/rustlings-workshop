use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
    #[clap(short, long)]
    select: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let mut num_fruits = opts.number;
    let select = opts.select;


    // Create the fruit salad
    let mut fruits = create_fruit_salad(num_fruits);

    // Challenge(3): Handle invalid number of fruits 
    if num_fruits > fruits.len() {
        println!("Error: Cannot generate {} fruits. There are only {} fruits in the salad.", num_fruits, fruits.len());
        std::process::exit(1);
    }

    // Challenge(1): Select the fruits the user wants
    if select {
        let mut selection = Vec::new();
        loop {
            // Print all the fruits in a list format for the user with indexes
            for (i, fruit) in fruits.iter().enumerate() {
                println!("{}: {}", i + 1, fruit);
            }
            // Ask the user a number for the desired fruit
            let mut input = String::new();
            println!("Enter the number of the fruit you want (enter 0 to stop): ");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => { println!("Error: Invalid input. Please enter a number."); 0 }
            };
            if input == 0 {
                break;
            }
            selection.push(fruits.remove(input - 1));
        }
        num_fruits = selection.len();
        fruits = selection;
    } 

    // Print the fruit salad in human readable format with a count of fruits used
    fruits.sort(); // Challenge(2): Sort the fruit salad
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits,
        fruits,
    );
}

