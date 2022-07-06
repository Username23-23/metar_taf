//TODO: cleanup- reduce repetitive calls, better on borrowing/refs, better error handling, format string slices
pub use std::ops::Range;
pub trait Parser {
    fn parse(info: String) -> String;
}
pub struct When;
impl Parser for When {
    fn parse(info: String) -> String {
        let day = info[..2].parse::<u32>().expect("Error parsing day METAR was observed");
        let time = info[2..6].parse::<u32>().expect("Error parsing time METAR was observed");
        let hour = time / 100;
        format!("Taken on the {}th day of the current month at {}:{} UTC\n", day, hour, time - (hour * 100))
    }
}
pub struct Wind;
impl Parser for Wind {
    fn parse(info: String) -> String {
        let mut parsed = String::new();
        let d = info[..=2].parse::<u32>(); // err here means dir was "VRB"
        let sp = info.find(" "); // some here means variable dir was included
        let g = info.find("G"); // some here means there is gust\
        let k = info.find("K").expect("Couldn't find wind speed");
        if let Ok(direction) = d {
            if let Some(i_s) = sp {
                parsed += format!("Dominant wind direction: {} degrees (varying between {} and {} degrees)\n", direction, info[i_s + 1..info.find("V").unwrap()].parse::<u32>().expect("Couldn't parse varying wind direction"), info[info.find("V").unwrap() + 1..].parse::<u32>().expect("Couldn't parse varying wind direction")).as_str();
            } else {
                parsed += format!("Wind direction: {} degrees\n", direction).as_str();
            }
        } else {
            parsed.push_str("Wind direction: variable\n");
        }
        if let Some(gust) = g {
            parsed += format!("Wind speed: {} knots, with gusts of {} knots", info[3..gust].parse::<u32>().expect("Couldn't parse wind speed"), info[gust + 1..k].parse::<u32>().expect("Couldn't parse wind gust speed")).as_str();
        } else {
            parsed += format!("Wind speed: {} knots", info[3..k].parse::<u32>().expect("Couldn't parse wind speed")).as_str();
        }
        parsed
    }
}
//TODO: fractions for visibility
pub enum Visibility {
    Plus(i32),
    Exact(i32), 
    Less(i32), 
}
impl Visibility {
    pub fn new(info: String) -> Self {
        if(&info[0..1] == "P") {
            Visibility::Plus(info[1..info.find("S").unwrap()].parse::<i32>().unwrap())
        } else if(&info[0..1] == "M") {
            Visibility::Less(info[1..info.find("S").unwrap()].parse::<i32>().unwrap())
        } else {
            Visibility::Exact(info[0..info.find("S").unwrap()].parse::<i32>().unwrap())
        }
    }
    pub fn visibility_for_rvr(info: String) -> Self {
        match &info[0..1] {
            "M" => Visibility::Less(info[info.find("M").unwrap() + 1..].parse::<i32>().unwrap()),
            "P" => Visibility::Plus(info[info.find("P").unwrap() + 1..].parse::<i32>().unwrap()),
            _ => Visibility::Exact(info[0..].parse::<i32>().unwrap()),
        }
    }
}
impl Parser for Visibility {
    fn parse(info: String) -> String {
        let mut parsed = String::new();
        let sm = info.find("S").expect("Couldn't parse visibility");
        let p = info.find("P");
        let m = info.find("M");
        if let Some(i_p) = p {
            parsed += format!("Visibiliy: more than {} statute miles", info[i_p + 1..sm].parse::<u32>().expect("Couldn't parse visibility")).as_str();
        } else if let Some(i_m) = m {
            parsed += format!("Visibiliy: less than {} statute miles", info[i_m + 1..sm].parse::<u32>().expect("Couldn't parse visibility")).as_str();
        } else {
            parsed += format!("Visibiliy: {} statute miles", info[..sm].parse::<u32>().expect("Couldn't parse visibility")).as_str();
        }
        parsed
    }
}
//TODO: vertical visib, clr/skc
pub struct CloudLayer;
impl Parser for CloudLayer {
    fn parse(info: String) -> String {
        let mut parsed = String::new();
        let height: u32 = info[3..6].parse::<u32>().expect("Couldn't parse cloud layer height") * 100; //what happens here if clr/skc (index out of bounds)
        match &info[0..=2] {
            "OVC" => parsed += format!("Overcast clouds at {} ft", height).as_str(),
            "BKN" => parsed += format!("Broken clouds at {} ft", height).as_str(),
            "SCT" => parsed += format!("Scattered clouds at {} ft", height).as_str(),
            "FEW" => parsed += format!("Few clouds at {} ft", height).as_str(),
            _ => parsed.push_str("Clear skies"),
        }
        parsed
    }
} 
pub struct Alt;
impl Parser for Alt {
    fn parse(info: String) -> String {
        let alt = (info[1..].parse::<f64>().expect("Couldn't parse altimeter setting")) / 100.0;
        format!("Altimiter: {} inHg\n", alt)
    }
}
pub struct Temps;
impl Parser for Temps {
    fn parse(info: String) -> String {
        let temp_celsius: i32;
        let dewpoint_celsius: i32;
        match info.len() {
            5 => {
                temp_celsius = info[..2].parse::<i32>().expect("Couldn't parse temperature");
                dewpoint_celsius = info[3..].parse::<i32>().expect("Couldn't parse dewpoint");
            },
            6 => {  
                temp_celsius = info[0..2].parse::<i32>().expect("Couldn't parse temperature");
                dewpoint_celsius = info[4..].parse::<i32>().expect("Couldn't parse dewpoint") - (2 * info[4..].parse::<i32>().expect("Couldn't parse dewpoint"));
            },
            7 => {
                temp_celsius = info[1..3].parse::<i32>().expect("Couldn't parse temperature")- (2 * info[1..3].parse::<i32>().expect("Couldn't parse temperature"));
                dewpoint_celsius = info[5..].parse::<i32>().expect("Couldn't parse dewpoint") - (2 * info[5..].parse::<i32>().expect("Couldn't parse dewpoint"));
            },
            _ => panic!("Unexpected length for temperature/dewpoint measurement")
        }
        format!("Temperature: {} Celsius\nDewpoint: {} Celsius\n", temp_celsius, dewpoint_celsius)
    }
}
pub struct Rvr;
impl Parser for Rvr {
    fn parse(info: String) -> String {
        let mut parsed = String::new();
        let slash = info.find("/").expect("Couldn't parse rvr measurement: \"/\" not found where expected");
        let f = info.find("F").expect("Couldn't parse rvr measurement: \"FT\" not found where expected");
        let v = info.find("V");
        let p = info.find("P");
        let m = info.find("M");
        parsed += format!("RVR for Runway {}: ", &info[1..slash]).as_str();
        if let Some(i_v) = v {
            if let Some(i_m) = m {
                parsed += format!("Between less than {} ft", info[i_m + 1..i_v].parse::<u32>().expect("Unable to parse rvr 1")).as_str();
            } else {
                parsed += format!("Less than {} ft", info[slash + 1..i_v].parse::<u32>().expect("Unable to parse rvr 2")).as_str();
            }
            if let Some(i_p) = p {
                parsed += format!("and more than {} ft", info[i_p + 1..f].parse::<u32>().expect("Unable to parse rvr 3")).as_str();
            } else {
                parsed += format!("and {} ft", info[i_v + 1..f].parse::<u32>().expect("Unable to parse rvr 4")).as_str();
            }
        } else {
            if let Some(i_m) = m {
                parsed += format!("Less than {} ft", info[i_m + 1..f].parse::<u32>().expect("Unable to parse rvr 5")).as_str();
            } else if let Some(i_p) = p {
                parsed += format!("More than {} ft", info[i_p + 1..f].parse::<u32>().expect("Unable to parse rvr 6")).as_str()
            } else {
                parsed += format!("{} ft", info[slash + 1..f].parse::<u32>().expect("Unable to parse rvr 7")).as_str();
            }
        }
        parsed
    }
}
pub struct Weather;
impl Parser for Weather {
    fn parse(info: String) -> String {
        let mut parsed = String::new();
        let mut current_index = 0;
        match &info[0..1] {
            "+" => {
                parsed.push_str("Heavy");
                current_index = 1;
            },
            "-" => {
                parsed.push_str("Light");
                current_index = 1;
            },
            _ => (),
        }
        while current_index < info.len() {
            match &info[current_index..=current_index + 1] {
                "VC" => parsed.push_str("In the vicinity"),
                "MI" => parsed.push_str("Shallow"),
                "PR" => parsed.push_str("Partial"),
                "BC" => parsed.push_str("Patches"),
                "DR" => parsed.push_str("Low Drifting"),
                "BL" => parsed.push_str("Blowing"),
                "SH" => parsed.push_str("Showers of"),
                "TS" => parsed.push_str("Thunderstorm"), 
                "FZ" => parsed.push_str("Freezing"),
                "DZ" => parsed.push_str("Drizzle"),
                "RA" => parsed.push_str("Rain"),
                "SN" => parsed.push_str("Snow"),
                "SG" => parsed.push_str("Snow Grains"),
                "IC" => parsed.push_str("Ice Crystals"),
                "PL" => parsed.push_str("Ice Pellets"),
                "GR" => parsed.push_str("Hail"),
                "GS" => parsed.push_str("Snow Pellets"),
                "UP" => parsed.push_str("Unknown Precipitation"),
                "BR" => parsed.push_str("Mist"),
                "FG" => parsed.push_str("Fog"),
                "FU" => parsed.push_str("Smoke"),
                "VA" => parsed.push_str("Volcanic ash"),
                "DU" => parsed.push_str("Widespread dust"), 
                "SA" => parsed.push_str("Sand"),
                "HZ" => parsed.push_str("Haze"),
                "PY" => parsed.push_str("Spray"),
                "PO" => parsed.push_str("Sand Whirls"),
                "SQ" => parsed.push_str("Squalls"),
                "FC" => parsed.push_str("Tornado"),
                "SS" => parsed.push_str("Sandstorm"),
                "DS" => parsed.push_str("Duststorm"), 
                _ => (),
            }
            current_index += 2;
        }
        parsed
    }
}
mod tests {
    use crate::us_metar_components::*;
    //paused due to upcoming refactor
   /* #[test]
    fn check_when() {
        let w = When::new(String::from("291314Z"));
        assert_eq!(w.day_of_month, 29);
        assert_eq!(w.zulu_time, 1314);
    }
    #[test]
    fn check_wind() {
        let a = Wind::new(String::from("08717G24KT"));
        assert_eq!(a.spd, (17..24));
        assert_eq!(a.dir, (087..087));
        let b = Wind::new(String::from("08717KT"));
        assert_eq!(b.spd, (17..17));
        assert_eq!(b.dir, (087..087));
        let c = Wind::new(String::from("08717G24KT 086V088"));
        assert_eq!(c.spd, (17..24));
        assert_eq!(c.dir, (086..088));
        let d = Wind::new(String::from("VRB03G05KT"));
        assert_eq!(d.spd, (3..5));
        assert_eq!(d.dir, (999..999));
    }
    #[test]
    fn check_alt() {
        let a = Alt::new(String::from("A2973"));
        assert_eq!(a.alt_inhg, 29.73);
    }
    #[test] 
    fn check_temps() {
        let a = Temps::new(String::from("17/14"));
        let b = Temps::new(String::from("07/M03"));
        let c = Temps::new(String::from("M09/M10"));
        assert_eq!(a.temp_celsius, 17);
        assert_eq!(a.dewpoint_celsius, 14);
        assert_eq!(b.temp_celsius, 7);
        assert_eq!(b.dewpoint_celsius, -3);
        assert_eq!(c.temp_celsius, -9);
        assert_eq!(c.dewpoint_celsius, -10);
    }
    #[test]
    fn check_weather() {

    }
    #[test]
    fn check_visibility() {
        let a = Visibility::new(String::from("9SM"));
        let b = Visibility::new(String::from("M6SM"));
        let c = Visibility::new(String::from("P4SM"));
        let get_range = |p: Visibility| -> i32 {
            match p {
                Visibility::Plus(a) => a + 1,
                Visibility::Exact(b) => b,
                Visibility::Less(c) => c - 1,
            }
        };
        assert_eq!(9, get_range(a));
        assert_eq!(5, get_range(b));
        assert_eq!(5, get_range(c));
    }
    #[test]
    //needs rewrite
    fn check_clouds() {
        let a = Cloud_layer::new(String::from("SCT036"));
        assert_eq!({
            match a {
                Cloud_layer::Sct(b) => b,
                _ => 0
            }
        }, 3600);
    }
    #[test]
    fn check_rvr() {
        let t_v = |a: Visibility| -> String {
            match a {
                Visibility::Plus(e) => format!("+{}", e),
                Visibility::Exact(f) => format!("{}", f),
                Visibility::Less(g) => format!("-{}", g),
            }
        };
        let a = Rvr::new(String::from("R05L/1600FT"));
        assert_eq!(a.rwy, String::from("05L"));
        assert_eq!(t_v(a.vis), String::from("1600"));
        let b = Rvr::new(String::from("R27/1500V1700FT"));
        assert_eq!(b.rwy, String::from("27"));
        assert_eq!(t_v(b.vis), String::from("1500"));
        assert_eq!(t_v(b.upper_bound.unwrap()), String::from("1700"));
        let c = Rvr::new(String::from("R31/M1400VP1600FT"));
        assert_eq!(c.rwy, String::from("31"));
        assert_eq!(t_v(c.vis), String::from("-1400"));
        assert_eq!(t_v(c.upper_bound.unwrap()), String::from("+1600"));
    } */
}