use enigo::{Direction::*, Enigo, Key, Keyboard, Settings};
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect args from SD
                                                   //setting up variables for potential arguments
    let mut command = "";

    for arg in &args[1..] {
        let lowercase_arg = arg.to_lowercase(); //lowercases the argument to avoid a capital letter breaking the macro
        let matched_command = match_command(&lowercase_arg); //Checks to see if argument is vaild command
        if !matched_command.is_empty() {
            //if the returned value is not "" will set the stratagem
            command = matched_command;
            break; // Break the loop if a matched command is found
        }
    }
    run_macro(command); //run the macro and pass over all variables
}

fn match_command(command: &str) -> &'static str {
    //checks to see if passed variable is a stratagem or not
    match command {
        "reinforce" => "wsdaw",
        "machine_gun_sentry" => "swddw",
        "gatling_sentry" => "swda",
        "mortar_sentry" => "swdds",
        "guard_dog" => "swawds",
        "autocannon_sentry" => "swdwaw",
        "rocket_sentry" => "swdda",
        "ems_mortar_sentry" => "swdsd",
        "anti-personnel_minefield" => "sawd",
        "supply_pack" => "saswws",
        "grenade_launcher" => "sawas",
        "laser_cannon" => "saswa",
        "incendiary_mines" => "saas",
        "gaz_mines" => "saaw",
        "guard_dog_rover" => "swawdd",
        "ballistic_shield_backpack" => "sawwd",
        "arc_thrower" => "sdwas",
        "shield_generator_pack" => "swadad",
        "orbital_precision_strike" => "ddw",
        "orbital_gas_strike" => "ddsd",
        "orbital_ems_strike" => "ddas",
        "orbital_smoke_strike" => "ddsw",
        "orbital_napalm_barrage" => "ddsadw",
        "hmg_emplacement" => "swadda",
        "shield_generation_relay" => "swasdd",
        "tesla_tower" => "swdwad",
        "eagle_strafing_run" => "wdd",
        "eagle_airstrike" => "wdsd",
        "eagle_cluster_bomb" => "wdssd",
        "eagle_napalm_airstrike" => "wdsw",
        "jump_pack" => "swwsw",
        "eagle_smoke_strike" => "wdws",
        "eagle_110mm_rocket_pods" => "wdwa",
        "eagle_500kg_bomb" => "wdsss",
        "orbital_gatling_barrage" => "dsaww",
        "orbital_airburst_strike" => "dd_d",
        "orbital_120mm_he_barrage" => "dsads",
        "orbital_380mm_he_barrage" => "dswwass",
        "orbital_walking_barrage" => "ddsads",
        "orbital_lasers" => "dswds",
        "orbital_railcannon_strike" => "dwssd",
        "machine_gun" => "saswd",
        "anti-material_rifle" => "sadws",
        "stalwart" => "saswwa",
        "expendable_anti-tank" => "ssawd",
        "recoiled_rifle" => "sadda",
        "flamethrower" => "sawsw",
        "autocannon" => "saswwd",
        "railgun" => "sdaswad",
        "spear" => "sswss",
        "wasp" => "sswsd",
        "commando" => "sawsd",
        "airburst" => "swwad",
        "sos_beacon" => "wsaw",
        "resupply" => "sswd",
        "eagle_rearm" => "wwawd",
        "hellbomb" => "swaswdsw",
        "prospecting_drill" => "ssadss",
        "super_earth_flag" => "swsw",
        "patriot_exosuit" => "asdwass",
        "seaf_artillery" => "dwws",
        "upload_data" => "adwww",
        "seismic_probe" => "wwaass",
        "orbital_illumination_flare" => "ddaa",
        _ => "", //returns blank if not so no keys will be pressed
    }
}
fn run_macro(command: &str) {
    const DELAY: u64 = 50;
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    //Input key gets pressed down

    enigo.key(Key::Control, Press).unwrap();
    sleep(Duration::from_millis(DELAY));

    for c in command.chars() {
        let key = parse_key(c);
        enigo.key(key, Press).unwrap();
        sleep(Duration::from_millis(DELAY));
        enigo.key(key, Release).unwrap();
        sleep(Duration::from_millis(DELAY));
    }
}

fn parse_key(key: char) -> Key {
    //converts wasd input to arrows
    match key {
        'w' => Key::UpArrow,
        's' => Key::DownArrow,
        'a' => Key::LeftArrow,
        'd' => Key::RightArrow,
        _ => Key::Control
    }
}
