fn main() {
    let song = [
        "A partridge in a pear tree.",
        "Two turtle doves",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for i in 0..12 {
        println!("On the {} day of Christmas", days[i]);
        println!("My true love gave to me");
        for j in (0..=i).rev() {
            if i > 0 && j == 0 {
                // Add "And" for the last item on days 2 through 12
                // Also lowercase the first letter to match grammar if needed,
                // or keep it as is.
                println!("And {}", song[j].to_lowercase());
            } else {
                println!("{}", song[j]);
            }
        }
        println!();
    }
}
