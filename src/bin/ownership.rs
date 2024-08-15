enum Light {
    Bright,
    Dull,
}

fn display_ligh(light: &Light) {
    match light {
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    }
}
// Rust uses ownership model in order to clean and leak free memory management, whoever(fn) is the owner of the variable, he is
// responsible to removing, the ownership can be transferred from 2 types , first is by moving and other is by borrowing

fn main() {
    let dull = Light::Dull;
    // display_ligh(dull);
    // display_ligh(dull);
    // the above two function when uncommented is giving an error, because when we initialize the variable, the ownership
    // is present into the main function, on calling display_ligh function with the variable , the ownership of
    // the dul variable is transfered to the display light function, now, this display light function is repsonsible for
    // cleaning the the original memory of dull, which will be cleared after the function execution completed.
    // so the next time we are calling that function the memory is no longer available.

    display_ligh(&dull);
    display_ligh(&dull);

    // In the above case the memory is not moved, instead the display light function is borrowed the memory and is not responsible
    // for clearning it, this will only clear once the function execution is finished.
}
