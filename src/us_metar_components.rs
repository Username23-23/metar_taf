//TODO: cleanup- reduce repetitive calls, better on borrowing/refs, better error handling, format string slices
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
pub struct Visibility;
impl Parser for Visibility {
    fn parse(info: String) -> String {
        let mut parsed = String::new();
        let sm = info.find("S").expect("Couldn't parse visibility");
        let p = info.find("P");
        let m = {
            if(&info[0..1] == "M") {
                Some(0)
            } else {
                None
            }
        };
        if let Some(i_p) = p {
            parsed += format!("Visibility: more than {} statute miles", info[i_p + 1..sm].parse::<u32>().expect("Couldn't parse visibility")).as_str();
        } else if let Some(i_m) = m {
            parsed += format!("Visibility: less than {} statute miles", info[i_m + 1..sm].parse::<u32>().expect("Couldn't parse visibility")).as_str();
        } else {
            parsed += format!("Visibility: {} statute miles", info[..sm].parse::<u32>().expect("Couldn't parse visibility")).as_str();
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
            "OVC" => parsed += format!("Overcast clouds at {} ft AGL", height).as_str(),
            "BKN" => parsed += format!("Broken clouds at {} ft AGL", height).as_str(),
            "SCT" => parsed += format!("Scattered clouds at {} ft AGL", height).as_str(),
            "FEW" => parsed += format!("Few clouds at {} ft AGL", height).as_str(),
            _ => parsed.push_str("No cloud layers observed"),
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
                parsed += format!("Between less than {} ft ", info[i_m + 1..i_v].parse::<u32>().expect("Unable to parse rvr 1")).as_str();
            } else {
                parsed += format!("Between {} ft ", info[slash + 1..i_v].parse::<u32>().expect("Unable to parse rvr 2")).as_str();
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
                parsed.push_str("Heavy ");
                current_index = 1;
            },
            "-" => {
                parsed.push_str("Light ");
                current_index = 1;
            },
            _ => (),
        }
        while current_index < info.len() {
            match &info[current_index..=current_index + 1] {
                "VC" => parsed.push_str("In the vicinity, "),
                "MI" => parsed.push_str("Shallow "),
                "PR" => parsed.push_str("Partial "),
                "BC" => parsed.push_str("Patches "),
                "DR" => parsed.push_str("Low Drifting "),
                "BL" => parsed.push_str("Blowing "),
                "SH" => parsed.push_str("Showers of "),
                "TS" => parsed.push_str("Thunderstorm(s) "), 
                "FZ" => parsed.push_str("Freezing "),
                "DZ" => parsed.push_str("Drizzle "),
                "RA" => parsed.push_str("Rain "),
                "SN" => parsed.push_str("Snow "),
                "SG" => parsed.push_str("Snow Grains "),
                "IC" => parsed.push_str("Ice Crystals "),
                "PL" => parsed.push_str("Ice Pellets "),
                "GR" => parsed.push_str("Hail "),
                "GS" => parsed.push_str("Snow Pellets "),
                "UP" => parsed.push_str("Unknown Precipitation "),
                "BR" => parsed.push_str("Mist "),
                "FG" => parsed.push_str("Fog "),
                "FU" => parsed.push_str("Smoke "),
                "VA" => parsed.push_str("Volcanic ash "),
                "DU" => parsed.push_str("Widespread dust "), 
                "SA" => parsed.push_str("Sand "),
                "HZ" => parsed.push_str("Haze "),
                "PY" => parsed.push_str("Spray "),
                "PO" => parsed.push_str("Sand Whirls "),
                "SQ" => parsed.push_str("Squalls "),
                "FC" => parsed.push_str("Tornado "),
                "SS" => parsed.push_str("Sandstorm "),
                "DS" => parsed.push_str("Duststorm "), 
                _ => (),
            }
            current_index += 2;
        }
        parsed
    }
}
mod tests {
    use crate::us_metar_components::*;
    #[test]
    fn when() {
        let w = When::parse(String::from("291314Z"));
        let s = String::from("Taken on the 29th day of the current month at 13:14 UTC\n");
        assert_eq!(w, s);
    }
    #[test]
    fn wind() {
        let a = Wind::parse(String::from("08717G24KT"));
        let s1 = String::from("Wind direction: 87 degrees\nWind speed: 17 knots, with gusts of 24 knots");
        assert_eq!(a, s1);
        let b = Wind::parse(String::from("08717KT"));
        let s2 = String::from("Wind direction: 87 degrees\nWind speed: 17 knots");
        assert_eq!(b, s2);
        let c = Wind::parse(String::from("08717G24KT 086V088"));
        let s3 = String::from("Dominant wind direction: 87 degrees (varying between 86 and 88 degrees)\nWind speed: 17 knots, with gusts of 24 knots");
        assert_eq!(c, s3);
        let d = Wind::parse(String::from("VRB03G05KT"));
        let s4 = String::from("Wind direction: variable\nWind speed: 3 knots, with gusts of 5 knots");
        assert_eq!(d, s4);
    }
    #[test]
    fn visibility() {
        let a = Visibility::parse(String::from("9SM"));
        let s1 = String::from("Visibility: 9 statute miles");
        assert_eq!(a, s1);
        let b = Visibility::parse(String::from("M6SM"));
        let s2 = String::from("Visibility: less than 6 statute miles");
        assert_eq!(b, s2);
        let c = Visibility::parse(String::from("P4SM"));
        let s3 = String::from("Visibility: more than 4 statute miles");
        assert_eq!(c, s3);
    }
    #[test]
    fn cloud_layer() {
        let a = CloudLayer::parse(String::from("SCT036"));
        let s = String::from("Scattered clouds at 3600 ft AGL");
        assert_eq!(a, s);
    }
    #[test]
    fn altimeter() {
        let a = Alt::parse(String::from("A2973"));
        let s = String::from("Altimiter: 29.73 inHg\n");
        assert_eq!(a, s);
    }
    #[test]
    fn rvr() {
        let a = Rvr::parse(String::from("R05L/1600FT"));
        let s1 = String::from("RVR for Runway 05L: 1600 ft");
        assert_eq!(a, s1);
        let b = Rvr::parse(String::from("R27/1500V1700FT"));
        let s2 = String::from("RVR for Runway 27: Between 1500 ft and 1700 ft");
        assert_eq!(b, s2);
        let c = Rvr::parse(String::from("R31/M1400VP1600FT"));
        let s3 = String::from("RVR for Runway 31: Between less than 1400 ft and more than 1600 ft");
        assert_eq!(c, s3);
    }
    #[test]
    fn temps() {
        let a = Temps::parse(String::from("17/14"));
        let s1 = String::from("Temperature: 17 Celsius\nDewpoint: 14 Celsius\n");
        assert_eq!(a, s1);
        let b = Temps::parse(String::from("07/M03"));
        let s2 = String::from("Temperature: 7 Celsius\nDewpoint: -3 Celsius\n");
        assert_eq!(b, s2);
        let c = Temps::parse(String::from("M09/M10"));
        let s3 = String::from("Temperature: -9 Celsius\nDewpoint: -10 Celsius\n");
        assert_eq!(c, s3);
    }
    #[test] 
    fn weather() {
        let a = Weather::parse(String::from("+FZRA"));
        let s1 = String::from("Heavy Freezing Rain ");
        assert_eq!(a, s1);
        let b = Weather::parse(String::from("SN"));
        let s2 = String::from("Snow ");
        assert_eq!(b, s2);
        let c = Weather::parse(String::from("VCTS"));
        let s3 = String::from("In the vicinity, Thunderstorm(s) ");
        assert_eq!(c, s3);
    }
}