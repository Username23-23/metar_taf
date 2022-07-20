//TODO: cleanup- reduce repetitive calls, better on borrowing/refs, better error handling, format string slices
//This file contains items needed to parse encoded groups that are found in METAR reports in countries other than the US and Canada
pub trait EncodedGroup {
    fn parse(raw_data: &String) -> String;
}
pub struct When;
impl EncodedGroup for When {
    fn parse(raw_data: &String) -> String {
        let day = &raw_data[..=1].parse::<u32>().expect("Error parsing day METAR was observed");
        let hour = &raw_data[2..=3].parse::<u32>().expect("Error parsing time METAR was observed");
        let minute = &raw_data[4..=5].parse::<u32>().expect("Error parsing time METAR was observed");
        format!("Taken on the {}th day of the current month at {}:{} UTC", day, hour, minute)
    }
}
pub struct Wind;
impl EncodedGroup for Wind {
    fn parse(raw_data: &String) -> String {
        let mut parsed = String::new();
        let d = raw_data[..=2].parse::<u32>(); // err here means dir was "VRB"
        let g = raw_data.find("G"); // some here means there is gust\
        let k = raw_data.find("K").expect("Couldn't find wind speed");
        if let Ok(direction) = d {
            parsed += format!("Wind direction: {} degrees\n", direction).as_str();
        } else {
            parsed.push_str("Wind direction: variable\n");
        }
        if let Some(gust) = g {
            parsed += format!("Wind speed: {} knots, with gusts of {} knots", raw_data[3..gust].parse::<u32>().expect("Couldn't parse wind speed"), raw_data[gust + 1..k].parse::<u32>().expect("Couldn't parse wind gust speed")).as_str();
        } else {
            parsed += format!("Wind speed: {} knots", raw_data[3..k].parse::<u32>().expect("Couldn't parse wind speed")).as_str();
        }
        parsed
    }
}
pub struct VariableWindDirection;
impl EncodedGroup for VariableWindDirection {
    fn parse(raw_data: &String) -> String {
        let v = raw_data.find("V").expect("Couldn't parse varying wind direction");
        format!("Wind direction varying between {} and {} degrees", raw_data[..v].parse::<u32>().expect("Couldn't parse varying wind direction"), raw_data[v + 1..].parse::<u32>().expect("Couldn't parse varying wind direction"))
    }
}
pub struct Temps;
impl EncodedGroup for Temps {
    fn parse(raw_data: &String) -> String {
        let temp_celsius: i32;
        let dewpoint_celsius: i32;
        match raw_data.len() {
            5 => {
                temp_celsius = raw_data[..2].parse::<i32>().expect("Couldn't parse temperature");
                dewpoint_celsius = raw_data[3..].parse::<i32>().expect("Couldn't parse dewpoint");
            },
            6 => {  
                temp_celsius = raw_data[0..2].parse::<i32>().expect("Couldn't parse temperature");
                dewpoint_celsius = raw_data[4..].parse::<i32>().expect("Couldn't parse dewpoint") - (2 * raw_data[4..].parse::<i32>().expect("Couldn't parse dewpoint"));
            },
            7 => {
                temp_celsius = raw_data[1..3].parse::<i32>().expect("Couldn't parse temperature")- (2 * raw_data[1..3].parse::<i32>().expect("Couldn't parse temperature"));
                dewpoint_celsius = raw_data[5..].parse::<i32>().expect("Couldn't parse dewpoint") - (2 * raw_data[5..].parse::<i32>().expect("Couldn't parse dewpoint"));
            },
            _ => panic!("Unexpected length for temperature/dewpoint measurement")
        }
        format!("Temperature: {} Celsius\nDewpoint: {} Celsius", temp_celsius, dewpoint_celsius)
    }
}
pub struct Weather;
impl EncodedGroup for Weather {
    fn parse(raw_data: &String) -> String {
        if(&raw_data[..] == "NSW") {
            return String::from("No significant weather");
        }
        let mut parsed = String::new();
        let mut current_index = 0;
        match &raw_data[0..1] {
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
        while current_index < raw_data.len() {
            match &raw_data[current_index..=current_index + 1] {
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
impl EncodedGroup for CloudLayer {
    fn parse(raw_data: &String) -> String {
        if(&raw_data[..] == "NSC" || &raw_data[..] == "NCD") {
            String::from("No cloud layers observed")
        } else {
            let mut parsed = String::new();
            let height = raw_data[3..6].parse::<u32>(); //err-> height reported as "///"
            if let Ok(hgt) = height {
                match &raw_data[0..=2] {
                    "OVC" => parsed += format!("Overcast clouds at {} ft AGL", hgt * 100).as_str(),
                    "BKN" => parsed += format!("Broken clouds at {} ft AGL", hgt * 100).as_str(),
                    "SCT" => parsed += format!("Scattered clouds at {} ft AGL", hgt* 100).as_str(),
                    "FEW" => parsed += format!("Few clouds at {} ft AGL", hgt * 100).as_str(),
                    _ => parsed.push_str("No cloud layers observed"),
                }
            } else {
                match &raw_data[0..=2] {
                    "OVC" => parsed.push_str("Overcast clouds, height not reported"),
                    "BKN" => parsed.push_str("Broken clouds, height not reported"),
                    "SCT" => parsed.push_str("Scattered clouds, height not reported"),
                    "FEW" => parsed.push_str("Few clouds, height not reported"),
                    _ => parsed.push_str("No cloud layers observed"),
                }
            }
            if(raw_data.len() > 6) {
                match &raw_data[6..=7] {
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
impl EncodedGroup for VerticalVisibility {
    fn parse(raw_data: &String) -> String {
        format!("Vertical visibility: {} ft", &raw_data[2..].parse::<u32>().expect("Couldn't parse vertical visibility") * 100)
    }
}
pub struct Qnh;
impl EncodedGroup for Qnh {
    fn parse(raw_data: &String) -> String {
        let qnh: u32 = raw_data[1..].parse::<u32>().expect("Could not parse QNH group");
        format!("QNH: {} hPa", qnh)
    }
}
pub struct Visibility;
impl EncodedGroup for Visibility {
    fn parse(raw_data: &String) -> String {
        let visib_m = raw_data[..].parse::<u32>().expect("Couldn't parse visibility");
        if(visib_m == 9999) {
            return String::from("Visibility: 10 km or more");
        }
        format!("Visibility: {} meters", visib_m)
    }
}
pub struct DirectionalVisibility;
impl EncodedGroup for DirectionalVisibility {
    fn parse(raw_data: &String) -> String {
        let dir_visib_m = raw_data[..4].parse::<u32>().expect(/*"Couldn't parse directional visibility "*/&raw_data[..]);
        match &raw_data[4..] {
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
impl EncodedGroup for Rvr {
    fn parse(raw_data: &String) -> String {
        //trend is broken
        let mut parsed = String::new();
        let slash = raw_data.find("/").expect("Couldn't parse rvr measurement: \"/\" not found where expected");
        let p = raw_data.find("P");
        let m = raw_data.find("M");
        parsed.push_str(format!("RVR for Runway {}: ", &raw_data[1..slash]).as_str());
        if let Some(i_p) = p {
            parsed.push_str(format!("more than {} meters", raw_data[slash + 1..=slash + 4].parse::<u32>().expect("Unable to parse rvr")).as_str());
            let len = raw_data[slash + 1..].len();
            if(len > 5) {
                match &raw_data[raw_data.len() - 1..] {
                    "U" => parsed.push_str("(increasing)"),
                    "D" => parsed.push_str("(decreasing)"),
                    "N" => parsed.push_str("(no trend observed)"),
                    _ => (),
                };
            }
        } else if let Some(i_m) = m {
            parsed.push_str(format!("less than {} meters", raw_data[slash + 1..=slash + 4].parse::<u32>().expect("Unable to parse rvr")).as_str());
            let len = raw_data[slash + 1..].len();
            if(len > 5) {
                match &raw_data[raw_data.len() - 1..] {
                    "U" => parsed.push_str("(increasing)"),
                    "D" => parsed.push_str("(decreasing)"),
                    "N" => parsed.push_str("(no trend observed)"),
                    _ => (),
                };
            }
        } else {
            parsed.push_str(format!("{} meters", raw_data[slash + 1..=slash + 4].parse::<u32>().expect("Unable to parse rvr")).as_str());
            let len = raw_data[slash + 1..].len();
            if(len > 5) {
                match &raw_data[raw_data.len() - 1..] {
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
pub struct RecentWeather;
impl EncodedGroup for RecentWeather {
    fn parse(raw_data: &String) -> String {
        format!("Recent weather: {}", Weather::parse(&String::from(&raw_data[2..])))
    }
}
pub struct WindshearInformation;
impl EncodedGroup for WindshearInformation {
    fn parse(raw_data: &String) -> String {
        let r = raw_data.find("R");
        if let Some(i_r) = r {
            format!("Windshear on Runway {}", &raw_data[i_r + 1..])
        } else {
            String::from("Windshear on all runways")
        }
    }
}
pub struct SeaInfo;
impl EncodedGroup for SeaInfo {
    fn parse(raw_data: &String) -> String {
        let slash = raw_data.find("/").expect("Unable to parse sea raw_data");
        let temp = &raw_data[1..slash].parse::<u32>().expect("Unable to find sea surface temperature");
        let h = raw_data.find("H");
        if let Some(i_h) = h {
            let wave_height = &raw_data[slash + 2..].parse::<u32>().expect("Unable to parse wave height");
            format!("Sea surface temperature: {} degrees Celsius, Wave height: {} decimeters", temp, wave_height)
        } else {
            match &raw_data[slash + 2..] {
                "0" => format!("Sea surface temperature: {} degrees Celsius, Sea calm and glassy", temp),
                "1" => format!("Sea surface temperature: {} degrees Celsius, Sea calm and rippled", temp),
                "2" => format!("Sea surface temperature: {} degrees Celsius, Sea smooth with wavelets", temp),
                "3" => format!("Sea surface temperature: {} degrees Celsius, Sea slight", temp),
                "4" => format!("Sea surface temperature: {} degrees Celsius, Sea moderate", temp),
                "5" => format!("Sea surface temperature: {} degrees Celsius, Sea rough", temp),
                "6" => format!("Sea surface temperature: {} degrees Celsius, Sea very rough", temp),
                "7" => format!("Sea surface temperature: {} degrees Celsius, Sea high", temp),
                "8" => format!("Sea surface temperature: {} degrees Celsius, Sea very high", temp),
                "9" => format!("Sea surface temperature: {} degrees Celsius, Sea phenomenal", temp),
                _ => format!("Sea surace temperature: {} degrees Celsius", temp),
            }
        }
    }
}
pub struct RunwayState;
impl EncodedGroup for RunwayState {
    fn parse(raw_data: &String) -> String {
        if(&raw_data[2..] == "SNOCLO") {
            return String::from("Runway closed due to snow");
        }
        let slash = raw_data.find("/").expect("Unable to parse runway state");
        let mut parsed = String::new();
        parsed.push_str("Runway ");
        parsed.push_str(&raw_data[1..slash]);
        //0919, 0519, 1079 and 0366
        match &raw_data[slash + 1..slash + 2] {
            "0" => parsed.push_str(" deposits: clear and dry\n"),
            "1" => parsed.push_str(" deposits: damp\n"),
            "2" => parsed.push_str(" deposits: water patches\n"),
            "3" => parsed.push_str(" deposits: rime and frost\n"),
            "4" => parsed.push_str(" deposits: dry snow\n"),
            "5" => parsed.push_str(" deposits: wet snow\n"),
            "6" => parsed.push_str(" deposits: slush\n"),
            "7" => parsed.push_str(" deposits: ice"),
            "8" => parsed.push_str(" deposits: compacted/rolled snow\n"),
            "9" => parsed.push_str(" deposits: frozen nuts or ridges\n"),
            "/" => parsed.push_str(" deposits: type not reported\n"),
            _ => (),
        }
        match &raw_data[slash + 2..slash + 3] {
            "1" => parsed.push_str("Extent of contamination: Less than 10% of runway\n"),
            "2" => parsed.push_str("Extent of contamination: 11-25% of runway\n"),
            "5" => parsed.push_str("Extent of contamination: 26-50% of runway\n"),
            "9" => parsed.push_str("Extent of contamination: More than 51% of runway\n"),
            "/" => parsed.push_str("Extent of contamination: not reported\n"),
            _ => (),
        }
        let depth = &raw_data[slash + 3..=slash + 4].parse::<u32>();
        if let Ok(d) = depth {
            if(*d <= 91) {
                parsed.push_str("Deposit depth: ");
                parsed.push_str(&raw_data[slash + 3..=slash + 4]);
                parsed.push_str(" milimeters\n");
            } else {
                match d {
                    92 => parsed.push_str("Deposit depth: 10 cm\n"), 
                    93 => parsed.push_str("Deposit depth: 15 cm\n"),
                    94 => parsed.push_str("Deposit depth: 20 cm\n"),
                    95 => parsed.push_str("Deposit depth: 25 cm\n"),
                    96 => parsed.push_str("Deposit depth: 30 cm\n"),
                    97 => parsed.push_str("Deposit depth: 35 cm\n"),
                    98 => parsed.push_str("Deposit depth: 40 cm\n"),
                    99 => parsed.push_str("Runway inoperational due to deposit\n"),
                    _  => parsed.push_str(""),
                }
            }
        } else {
            parsed.push_str("Unreported deposit depth\n");
        }
        return parsed;
    }
}
pub struct TrendTime;
impl EncodedGroup for TrendTime {
    fn parse(raw_data: &String) -> String {
        let hour = &raw_data[2..=3].parse::<u32>().expect("Unable to parse trend time");
        let minute = &raw_data[4..].parse::<u32>().expect("Unable to parse trend time");
        match &raw_data[0..=1] {
            "FM" => format!("From {}:{} UTC", hour, minute),
            "AT" => format!("At {}:{} UTC", hour, minute),
            "TL" => format!("Until {}:{} UTC", hour, minute),
            _ => String::new()
        }
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