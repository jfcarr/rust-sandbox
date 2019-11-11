mod planetdata;

fn main() {
    let (planet_info, status) = planetdata::get_planet_info_vector("Venus".to_string());

    if status == "OK" {
        println!("Found {}.  Axis is {}.", planet_info.name, planet_info.axis);
    } else {
        println!("{} not found.", planet_info.name);
    }
}
