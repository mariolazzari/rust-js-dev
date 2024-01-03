fn main() {
    let years: [i32; 3] = [1975, 2000, 2023];

    // for loop
    for year in years.iter() {
        println!("year: {}", year);
    }

    let mut new_year_countdown = 10;

    // while loop
    while new_year_countdown > 0 {
        println!("{new_year_countdown}");
        new_year_countdown -= 1;
    }
}
