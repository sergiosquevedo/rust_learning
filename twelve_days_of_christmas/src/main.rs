fn main() {
    println!("~~~~~~~~~~~~~~~~ TWELVE DAYS OF CHRISTMAS ~~~~~~~~~~~~~~~~\n");

    let song_letter = initialize_song_days();
    let activities = song_letter.map(|x| x.1 );

    for (index, day) in song_letter.iter().enumerate() {
        println!("[Verse {}]", index+1);
        print_day_verse(*day);
        print_other_days_activity(index, activities);
    };
}

fn print_other_days_activity(index: usize, activities: [&str; 12]){
    for i in (0..index).rev() {
        println!("{}", activities[i]);
    };
    println!();
}

fn print_day_verse(day: (&str, &str)){
    println!("On the {} day of Christmas, my true love sent to me", day.0);
    println!("{}", day.1)
}

fn initialize_song_days() -> [(&'static str, &'static str); 12] {
    let first_day = ("first", "A partridge in a pear tree!");
    let second_day = ("second", "Two turtle doves and");
    let third_day = ("third", "Three french hens");
    let fourth_day = ("fourth", "Four calling birds");
    let fifth_day = ("fifth", "Five golden rings");
    let sixth_day = ("sixth", "Six geese a-laying");
    let seventh_day = ("seventh", "Seven swans a-swimming");
    let eighth_day = ("eighth", "Eight maids a-milking");
    let ninth_day = ("ninth", "Nine ladies dancing");
    let tenth_day = ("tenth", "Ten lords a-leaping");
    let eleventh_day = ("eleventh", "Eleven pipers piping");
    let twelfth_day = ("twelfth", "Twelve drummers drumming");

    [
        first_day,
        second_day,
        third_day,
        fourth_day,
        fifth_day,
        sixth_day,
        seventh_day,
        eighth_day,
        ninth_day,
        tenth_day,
        eleventh_day,
        twelfth_day,
    ]
}
