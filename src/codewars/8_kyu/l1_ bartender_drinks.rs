fn main() {
    let result: &str = get_drink_by_profession("jabroni");
    println!("{}", result);
}

fn get_drink_by_profession(param: &str) -> &'static str {
    return match param.to_lowercase().as_str() {
        "jabroni" => "Patron Tequila",
        "school counselor" => "Anything with Alcohol",
        "programmer" => "Hipster Craft Beer",
        "bike gang member" => "Moonshine",
        "politician" => "Your tax dollars",
        "rapper" => "Cristal",
        _ => "Beer",
    }
}