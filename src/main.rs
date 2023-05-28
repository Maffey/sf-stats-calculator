use std::io;

fn main() {
    println!("How many points do you have allocated in your primary attribute?");

    let mut primary_attribute: String = String::new();

    io::stdin()
        .read_line(&mut primary_attribute)
        .expect("Failed to read the attribute.");

    let primary_attribute: f64 = match primary_attribute.trim().parse() {
        Ok(value) => value,
        Err(_) => panic!("Error while trying to parse the value!"),
    };

    let constitution = (primary_attribute * 0.8) as u32;
    let luck = (primary_attribute * 0.4) as u32;
    let secondary_attributes = (primary_attribute * 0.1) as u32;

    println!(
        "Main attribute: {primary_attribute}
Recommended attributes are:
Constitution: {constitution}
Luck: {luck}
Secondary attributes: {secondary_attributes}
Have fun playing!
Press ENTER to close the app."
    );

    io::stdin().read_line(&mut String::new()).unwrap();
}
