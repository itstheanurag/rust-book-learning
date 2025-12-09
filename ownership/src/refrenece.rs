pub fn ownership_refrences() {
    println!("Ownership refrences");
    let name: String = String::from("Gaurav Kumar");
    // this allows to pass the string refrences but it;s not updatable
    println!("Calculated length of the name: {}", calculate_length(&name));

    let occupation: String = String::from(" is a developer, i suppose");
    let full_string: String = format!("{}{}", name, occupation);
    println!("concatinated string is: {}", full_string);

    let mut hero: String = String::from("There are no heroes in this world");
    update_string(&mut hero);
    println!("{:?}", hero);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn update_string(s: &mut String) {
    s.push_str(" — Even so we must, believe that everything will be just fine.");
}
