fn main() {
    let years: [i32; 3] = [1995, 2000, 2005];

    let first_year = years[0];
    println!("First year: {}", first_year);

    let [_, second_year, third_year] = years;
    println!("Second year: {}", second_year);
    println!("Third year: {}", third_year);

    for year in years.iter() {
        println!("Next year: {}", year);
    }
}
