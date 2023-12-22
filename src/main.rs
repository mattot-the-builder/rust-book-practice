struct Car {
    model: String,
    year: u32,
    color: String,
    tire_size: u32,
}

fn main() {

    let mut proton = Car {
        model: String::from("proton Exora"),
        year: 2010,
        color: String::from("Black"),
        tire_size: 17,
    };

    println!("Car model : {}", proton.model);

    // Updating car model with new value
    proton.model = String::from("Proton Exora 2.0");

    println!("Updated car model : {}", proton.model);

    // create new car instance
    let perodua = create_car(String::from("Perodua Axia"), 2019, String::from("Red"), 15); 
    println!("Car model : {}", perodua.model);

    // using struct update syntax
    let honda = Car {
        model: String::from("Honda City"),
        ..proton
    };

    println!("Car model  and color : {}, {}", honda.model, honda.color);
}


// function to create struct instance
fn create_car(model: String, year: u32, color: String, tire_size: u32) -> Car {
    Car {
        // shorthand used because parameter and struct field has the same name
        model,
        year,
        color,
        tire_size,
    }
}
