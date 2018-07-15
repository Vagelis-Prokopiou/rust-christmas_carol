fn main() {
    let presents_per_day: Vec<(&str, &str)> = vec![
        ("first", "A Partridge in a Pear Tree"),
        ("second", "Two Turtle Doves"),
        ("third", "Three French Hens"),
        ("fourth", "Four Calling Birds"),
        ("fifth", "Five Golden Rings"),
        ("sixth", "Six Geese a Laying"),
        ("seventh", "Seven Swans a Swimming"),
        ("eighth", "Eight Maids a Milking"),
        ("ninth", "Nine Ladies Dancing"),
        ("tenth", "Ten Lords a Leaping"),
        ("eleventh", "Eleven Pipers Piping"),
        ("twelfth", "12 Drummers Drumming"),
    ];
    let mut result: String = String::new();

    for (index, pair) in presents_per_day.iter().enumerate() {
        if index == 1 {
            result = pair.1.to_string() + "\nand " + &result;
        } else {
            result = pair.1.to_string() + "\n" + &result;
        }
        println!("On the {} day of Christmas my true love sent to me:\n{}", pair.0, result);
    }
}
