/// Prints the lyrics to "The Twelve Days of Christmas".
/// It utilizes the repetitive and cumulative nature of the song.
fn print_twelve_days_of_christmas() {
    // Array holding the ordinal names for the days (1st to 12th)
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    // Array holding the gift descriptions for each day
    // Index 0 corresponds to the first day's gift, etc.
    let gifts = [
        "A partridge in a pear tree", // Special handling for the first day / "And"
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    // Loop through each day, from 1 to 12
    // (day_num represents the actual day number, 1-indexed)
    for day_num in 1..=12 {
        // Print the introductory line for the verse
        // Access the 'days' array using 0-based index (day_num - 1)
        println!("On the {} day of Christmas,", days[day_num - 1]);
        println!("My true love gave to me");

        // Loop *backwards* from the current day down to the first day
        // to print the cumulative list of gifts for that verse.
        // (gift_day_num represents the day associated with the gift being printed)
        for gift_day_num in (1..=day_num).rev() {
            // Get the 0-based index for the 'gifts' array
            let gift_index = gift_day_num - 1;

            // Handle the special case for the first gift ("A partridge...")
            if gift_day_num == 1 { // Is this the first day's gift line?
                if day_num == 1 { // Is it the *very first* verse?
                    // On the first day, just print the gift with a period.
                    println!("{}.", gifts[gift_index]);
                } else {
                    // On subsequent days, prepend "And" and add a period.
                    println!("And {}.", gifts[gift_index].to_lowercase()); // Often sung lowercase here
                }
            } else {
                // For all other gifts, print them with a comma.
                println!("{},", gifts[gift_index]);
            }
        }

        // Print a blank line for separation between verses
        println!();
    }
}

// Main function to execute the program
fn main() {
    println!("--- The Twelve Days of Christmas ---");
    println!(); // Add a space before starting the lyrics

    // Call the function to print the lyrics
    print_twelve_days_of_christmas();
}