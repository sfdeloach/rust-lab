// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

/// provided a number, returns the ordinal
fn ordinal(n: u32) -> &'static str {
    match (n % 10, n % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    }
}

/// provided a gift and a day, returns the corresponding verse of the song
fn gift_line(gift: u32, day: u32) -> &'static str {
    match (gift, day) {
        (1, 1) => "A partridge in a pear tree.\n",
        (1, 12) => "And a partridge in a pear tree!\n",
        (1, _) => "And a partridge in a pear tree.\n",
        (2, _) => "Two turtle doves,",
        (3, _) => "Three French hens,",
        (4, _) => "Four calling birds,",
        (5, _) => "Five golden rings,",
        (6, _) => "Six geese a-laying,",
        (7, _) => "Seven swans a-swimming,",
        (8, _) => "Eight maids a-milking,",
        (9, _) => "Nine ladies dancing,",
        (10, _) => "Ten lords a-leaping,",
        (11, _) => "Eleven pipers piping,",
        (12, _) => "Twelve drummers drumming,",
        _ => unreachable!("no gift for day {gift}"),
    }
}

fn main() {
    println!("--- The Twelve Days of Christmas ---\n");

    for day in 1..=12 {
        println!(
            "On the {day}{} day of Christmas,\nMy true love sent to me:",
            ordinal(day)
        );

        for gift in (1..=day).rev() {
            println!("{}", gift_line(gift, day));
        }
    }
}
