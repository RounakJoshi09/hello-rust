#[derive(Debug)]
enum ThingsInTheSky {
    Sun(String),
    Stars(i32),
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun!")),
        _ => ThingsInTheSky::Stars(52),
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) => println!("{description}"),
        ThingsInTheSky::Stars(n) => println!("{n}"),
    }
}

fn main() {
    let time = 8;
    let skystate = create_skystate(time);
    check_skystate(&skystate);

    let mut country_sky_state: Vec<ThingsInTheSky> = Vec::new();
    country_sky_state.push(create_skystate(12));
    country_sky_state.push(create_skystate(20));
    country_sky_state.push(create_skystate(30));
    println!("Country Sky State: {:?}", country_sky_state);
}
