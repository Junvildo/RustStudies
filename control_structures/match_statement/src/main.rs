fn main() {
    print_choice(Suit::Heart);
    print_choice(Suit::Club);
    print_choice(Suit::Diamond);
    print_choice(Suit::Spade);

    country(44);
    country(34);
    country(125);
    country(-15);
}

fn country(code:i32){
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=99 => "unknown",
        _ => "invalid"
    };
    println!("Country is {}", country);
}


enum Suit {
    Heart,
    Spade,
    Club,
    Diamond
}

fn print_choice(choice: Suit) {
    match choice {
        Suit::Heart => {println!("\u{2665}")}
        Suit::Spade => {println!("\u{2660}")}
        Suit::Club => {println!("\u{2663}")}
        Suit::Diamond => {println!("\u{2666}")}
    }
}
