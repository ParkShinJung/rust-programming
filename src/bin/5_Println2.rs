fn main() {
    let my_city = "Seoul";
    let year = 2025;
    let population = 9_700_000;
    println!("1. The city of {} in {} had a population of {}", my_city, year, population);
    println!("2. The city of {city} in {years} had a population of {populations}",
             city = my_city,
             years = year,
             populations = population);
    println!("3. The city of {0} in {1} had a population of {2}. I love {0}!",
             my_city,
             year,
             population);
}

