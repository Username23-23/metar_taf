//TODO: cleanup- reduce repetitive calls, better on borrowing/refs, better error handling, format string slices
//This file contains items needed to parse encoded groups that are found in METAR reports in countries other than the US and Canada
pub trait Parser {
    fn parse(info: &String) -> String;
}
pub struct When;
impl Parser for When {
    fn parse(info: &String) -> String {
        let day = info[..2].parse::<u32>().expect("Error parsing day METAR was observed");
        let time = info[2..6].parse::<u32>().expect("Error parsing time METAR was observed");
        let hour = time / 100;
        format!("Taken on the {}th day of the current month at {}:{} UTC", day, hour, time - (hour * 100))
    }
}
pub struct Wind;
impl Parser for Wind {
    fn parse(info: &String) -> String {
        let mut parsed = String::new();
        let d = info[..=2].parse::<u32>(); // err here means dir was "VRB"
        let g = info.find("G"); // some here means there is gust\
        let k = info.find("K").expect("Couldn't find wind speed");
        if let Ok(direction) = d {
            parsed += format!("Wind direction: {} degrees\n", direction).as_str();
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
pub struct VariableWindDirection;
impl Parser for VariableWindDirection {
    fn parse(info: &String) -> String {
        let v = info.find("V").expect("Couldn't parse varying wind direction");
        format!("Wind direction varying between {} and {} degrees", info[..v].parse::<u32>().expect("Couldn't parse varying wind direction"), info[v + 1..].parse::<u32>().expect("Couldn't parse varying wind direction"))
    }
}
pub struct Temps;
impl Parser for Temps {
    fn parse(info: &String) -> String {
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
        format!("Temperature: {} Celsius\nDewpoint: {} Celsius", temp_celsius, dewpoint_celsius)
    }
}
pub struct Weather;
impl Parser for Weather {
    fn parse(info: &String) -> String {
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
pub struct CloudLayer;
impl Parser for CloudLayer {
    fn parse(info: &String) -> String {
        if(&info[..] == "NSC" || &info[..] == "NCD") {
            String::from("No cloud layers observed")
        } else {
            let mut parsed = String::new();
            let height = info[3..6].parse::<u32>(); //err-> height reported as "///"
            if let Ok(hgt) = height {
                match &info[0..=2] {
                    "OVC" => parsed += format!("Overcast clouds at {} ft AGL", hgt * 100).as_str(),
                    "BKN" => parsed += format!("Broken clouds at {} ft AGL", hgt * 100).as_str(),
                    "SCT" => parsed += format!("Scattered clouds at {} ft AGL", hgt* 100).as_str(),
                    "FEW" => parsed += format!("Few clouds at {} ft AGL", hgt * 100).as_str(),
                    _ => parsed.push_str("No cloud layers observed"),
                }
            } else {
                match &info[0..=2] {
                    "OVC" => parsed.push_str("Overcast clouds, height not reported"),
                    "BKN" => parsed.push_str("Broken clouds, height not reported"),
                    "SCT" => parsed.push_str("Scattered clouds, height not reported"),
                    "FEW" => parsed.push_str("Few clouds, height not reported"),
                    _ => parsed.push_str("No cloud layers observed"),
                }
            }
            if(info.len() > 6) {
                match &info[6..=7] {
                    "CB" => parsed.push_str("\nThe layer above consists of cumulonimbus clouds"),
                    "TC" => parsed.push_str("\nThe layer above consists of towering cumulus clouds"),
                    _ => (),
                }
            }
            parsed
        }   
    }
}
pub struct VerticalVisibility;
impl Parser for VerticalVisibility {
    fn parse(info: &String) -> String {
        format!("Vertical visibility: {} ft", &info[2..].parse::<u32>().expect("Couldn't parse vertical visibility") * 100)
    }
}
pub struct Qnh;
impl Parser for Qnh {
    fn parse(info: &String) -> String {
        let qnh: u32 = info[1..].parse::<u32>().expect("Could not parse QNH group");
        format!("QNH: {} hPa", qnh)
    }
}
pub struct Visibility;
impl Parser for Visibility {
    fn parse(info: &String) -> String {
        let visib_m = info[..].parse::<u32>().expect("Couldn't parse visibility");
        if(visib_m == 9999) {
            return String::from("Visibility: 10 km or more");
        }
        format!("Visibility: {} meters", visib_m)
    }
}
pub struct DirectionalVisibility;
impl Parser for DirectionalVisibility {
    fn parse(info: &String) -> String {
        let dir_visib_m = info[..4].parse::<u32>().expect(/*"Couldn't parse directional visibility "*/&info[..]);
        match &info[4..] {
            "N" => format!("Visibility down to {} meters when looking north from station", dir_visib_m),
            "NE" => format!("Visibility down to {} meters when looking northeast from station", dir_visib_m),
            "E" => format!("Visibility down to {} meters when looking east from station", dir_visib_m),
            "SE" => format!("Visibility down to {} meters when looking southeast from station", dir_visib_m),
            "S" => format!("Visibility down to {} meters when looking south from station", dir_visib_m),
            "SW" => format!("Visibility down to {} meters when looking southwest from station", dir_visib_m),
            "W" => format!("Visibility down to {} meters when looking west from station", dir_visib_m),
            "NW" => format!("Visibility down to {} meters when looking northwest from station", dir_visib_m),
            _ => format!("Visibility down to {} meters in an unknown direction", dir_visib_m)
        }
    }
}
//TODO: cleaner solution for trend 
pub struct Rvr;
impl Parser for Rvr {
    fn parse(info: &String) -> String {
        //trend is broken
        let mut parsed = String::new();
        let slash = info.find("/").expect("Couldn't parse rvr measurement: \"/\" not found where expected");
        let p = info.find("P");
        let m = info.find("M");
        parsed.push_str(format!("RVR for Runway {}: ", &info[1..slash]).as_str());
        if let Some(i_p) = p {
            parsed.push_str(format!("more than {} meters", info[slash + 1..=slash + 4].parse::<u32>().expect("Unable to parse rvr")).as_str());
            let len = info[slash + 1..].len();
            if(len > 5) {
                match &info[info.len() - 1..] {
                    "U" => parsed.push_str("(increasing)"),
                    "D" => parsed.push_str("(decreasing)"),
                    "N" => parsed.push_str("(no trend observed)"),
                    _ => (),
                };
            }
        } else if let Some(i_m) = m {
            parsed.push_str(format!("less than {} meters", info[slash + 1..=slash + 4].parse::<u32>().expect("Unable to parse rvr")).as_str());
            let len = info[slash + 1..].len();
            if(len > 5) {
                match &info[info.len() - 1..] {
                    "U" => parsed.push_str("(increasing)"),
                    "D" => parsed.push_str("(decreasing)"),
                    "N" => parsed.push_str("(no trend observed)"),
                    _ => (),
                };
            }
        } else {
            parsed.push_str(format!("{} meters", info[slash + 1..=slash + 4].parse::<u32>().expect("Unable to parse rvr")).as_str());
            let len = info[slash + 1..].len();
            if(len > 5) {
                match &info[info.len() - 1..] {
                    "U" => parsed.push_str("(increasing)"),
                    "D" => parsed.push_str("(decreasing)"),
                    "N" => parsed.push_str("(no trend observed)"),
                    _ => (),
                };
            }
        }
        parsed
    }
}
mod tests {
    use crate::world_metar::*;
    #[test]
    fn when() {
        let w = When::parse(&String::from("291314Z"));
        let s = String::from("Taken on the 29th day of the current month at 13:14 UTC");
        assert_eq!(w, s);
    }
    #[test]
    fn wind() {
        let a = Wind::parse(&String::from("08717G24KT"));
        let s1 = String::from("Wind direction: 87 degrees\nWind speed: 17 knots, with gusts of 24 knots");
        assert_eq!(a, s1);
        let b = Wind::parse(&String::from("08717KT"));
        let s2 = String::from("Wind direction: 87 degrees\nWind speed: 17 knots");
        assert_eq!(b, s2);
        let c = Wind::parse(&String::from("VRB03G05KT"));
        let s3 = String::from("Wind direction: variable\nWind speed: 3 knots, with gusts of 5 knots");
        assert_eq!(c, s3);
    }
    #[test]
    fn cloud_layer() {
        let a = CloudLayer::parse(&String::from("SCT036"));
        let s = String::from("Scattered clouds at 3600 ft AGL");
        assert_eq!(a, s);
    }
    #[test]
    fn temps() {
        let a = Temps::parse(&String::from("17/14"));
        let s1 = String::from("Temperature: 17 Celsius\nDewpoint: 14 Celsius");
        assert_eq!(a, s1);
        let b = Temps::parse(&String::from("07/M03"));
        let s2 = String::from("Temperature: 7 Celsius\nDewpoint: -3 Celsius");
        assert_eq!(b, s2);
        let c = Temps::parse(&String::from("M09/M10"));
        let s3 = String::from("Temperature: -9 Celsius\nDewpoint: -10 Celsius");
        assert_eq!(c, s3);
    }
    #[test] 
    fn weather() {
        let a = Weather::parse(&String::from("+FZRA"));
        let s1 = String::from("Heavy Freezing Rain ");
        assert_eq!(a, s1);
        let b = Weather::parse(&String::from("SN"));
        let s2 = String::from("Snow ");
        assert_eq!(b, s2);
        let c = Weather::parse(&String::from("VCTS"));
        let s3 = String::from("In the vicinity, Thunderstorm(s) ");
        assert_eq!(c, s3);
    }
    #[test]
    fn varying_wind_dir() {
        let s = String::from("180V240");
        assert_eq!(String::from("Wind direction varying between 180 and 240 degrees"), VariableWindDirection::parse(&s));
    }
    #[test]
    fn vertical_visib() {
        let s = String::from("VV003");
        assert_eq!(VerticalVisibility::parse(&s), String::from("Vertical visibility: 300 ft"));
    }
}