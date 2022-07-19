//TODO: cleanup- reduce repetitive calls, better on borrowing/refs, better error handling, format string slices
//This file contains items needed to parse encoded groups that are found only in US METAR repots
use crate::world_metar::Parse;
pub trait ParseForRemarks {
    fn parse_rmk(info: &String) -> String; //to make it clear that a remark is being parsed
}
pub struct USVisibility;
impl Parse for USVisibility {
    fn parse(info: &String) -> String {
        let mut parsed = String::new();
        let sm = info.find("S").expect("Couldn't parse visibility");
        let p = info.find("P");
        let slash = info.find("/");
        let sp = info.find(" ");
        let m = {
            if(&info[0..1] == "M") {
                Some(0)
            } else {
                None
            }
        };
        let visibility_number: f64;
        //TODO: cut down on extremely repetitive logic
        if let Some(i_sl) = slash {
            if let Some(i_m) = m {
                if let Some(i_sp) = sp {
                    visibility_number = info[1..i_sp].parse::<f64>().expect("Visib") + (info[i_sp + 1..i_sl].parse::<f64>().expect("Visib") / info[i_sl + 1..sm].parse::<f64>().expect("Visib"));
                } else {
                    visibility_number = (info[1..i_sl].parse::<f64>().expect("Visib") / info[i_sl + 1..sm].parse::<f64>().expect("Visib"));
                }
            } else if let Some(i_p) = p {
                if let Some(i_sp) = sp {
                    visibility_number = info[1..i_sp].parse::<f64>().expect("Visib") + (info[i_sp + 1..i_sl].parse::<f64>().expect("Visib") / info[i_sl + 1..sm].parse::<f64>().expect("Visib"));
                } else {
                    visibility_number = (info[1..i_sl].parse::<f64>().expect("Visib") / info[i_sl + 1..sm].parse::<f64>().expect("Visib"));
                }
            } else {
                if let Some(i_sp) = sp {
                    visibility_number = info[0..i_sp].parse::<f64>().expect("Visib") + (info[i_sp + 1..i_sl].parse::<f64>().expect("Visib") / info[i_sl + 1..sm].parse::<f64>().expect("Visib"));
                } else {
                    visibility_number = (info[0..i_sl].parse::<f64>().expect("Visib") / info[i_sl + 1..sm].parse::<f64>().expect("Visib"));
                }
            }
        } else {
            if let Some(i_m) = m {
                visibility_number = info[1..sm].parse::<f64>().expect("Unable to parse visibility");
            } else if let Some(i_p) = p {
                visibility_number = info[1..sm].parse::<f64>().expect("Unable to parse visibility");
            } else {
                visibility_number = info[..sm].parse::<f64>().expect("Unable to parse visibility");
            }
        }
        if let Some(i_p) = p {
            parsed += format!("Visibility: more than {} statute miles", visibility_number).as_str();
        } else if let Some(i_m) = m {
            parsed += format!("Visibility: less than {} statute miles", visibility_number).as_str();
        } else {
            parsed += format!("Visibility: {} statute miles", visibility_number).as_str();
        }
        parsed
    }
}
pub struct USCloudLayer;
impl Parse for USCloudLayer {
    fn parse(info: &String) -> String {
        if(&info[..] == "CLR" || &info[..] == "SKC") {
            String::from("No cloud layers observed")
        } else {
            let mut parsed = String::new();
            let height: u32 = info[3..6].parse::<u32>().expect("Couldn't parse cloud layer height") * 100;
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
}
pub struct Alt;
impl Parse for Alt {
    fn parse(info: &String) -> String {
        let alt = (info[1..].parse::<f64>().expect("Couldn't parse altimeter setting")) / 100.0;
        format!("Altimiter: {} inHg\n", alt)
    }
}
pub struct USRvr;
impl Parse for USRvr {
    fn parse(info: &String) -> String {
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
pub struct SensorType;
impl ParseForRemarks for SensorType {
    fn parse_rmk(info: &String) -> String {
        match &info[2..] {
            "1" => String::from("The sensor used to observe the METAR report above is AO1, meaning it lacks a precipitation discriminant"),
            "2" => String::from("The sensor used to observe the METAR report above is AO2, meaning it contains a precipitation discriminant"),
            _ => String::from("Couldn't parse sensor type data"),
        }
    }
}
pub struct SeaLevelPressure;
impl ParseForRemarks for SeaLevelPressure {
    fn parse_rmk(info: &String) -> String {
        let raw = &info[3..].parse::<f64>();
        if let Ok(raw_f64) = raw {
            let slp: f64;
            match &info[3..4] {
                "9" => slp = (raw_f64 / 10.0) + 900.0,
                _ => slp = (raw_f64 / 10.0) + 1000.0,
            }
            format!("Sea level pressure: {} hPa", slp)
        } else {
            String::from("No sea level pressure")
        }
    }
}
pub struct AdditionalTemperatureData;
impl ParseForRemarks for AdditionalTemperatureData {
    fn parse_rmk(info: &String) -> String {
        let temp: f64;
        let dp: f64;
        match &info[1..2] {
            "0" => temp = &info[2..=4].parse::<f64>().expect("Error parsing temperature remark") / 10.0,
            "1" => temp = -(&info[2..=4].parse::<f64>().expect("Error parsing temperature remark")) / 10.0,
            _ => return String::new(),
        }
        match &info[5..6] {
            "0" => dp = &info[6..].parse::<f64>().expect("Error parsing temperature remark") / 10.0,
            "1" => dp = -(&info[6..].parse::<f64>().expect("Error parsing temperature remark")) / 10.0,
            _ => return String::new(),
        }
        format!("Temperature: {} degrees celsius, dewpoint: {} degrees celsius", temp, dp)
    }
}
pub struct PeakWind;
impl ParseForRemarks for PeakWind {
    fn parse_rmk(info: &String) -> String {
        let slash = info.find("/").expect("Unable to parse peak wind");
        let dir = &info[7..=9].parse::<u32>().expect("Unable to parse peak wind direction");
        let spd = &info[10..slash].parse::<u32>().expect("Unable to parse peak wind speed");
        if info.len() <= 16 {
            let min = &info[slash + 1..].parse::<u32>().expect("Unable to parse peak wind time");
            format!("Peak wind: {} degrees at {} knots, {} minutes from the current hour", dir, spd, min)
        } else {
            let hour = &info[slash + 1..=slash + 2].parse::<u32>().expect("Unable to parse peak wind time");
            let min = &info[slash + 3..].parse::<u32>().expect("Unable to parse peak wind time");
            format!("Peak wind: {} degrees at {} knots, {}:{} UTC", dir, spd, hour, min)
        }
    }
}
pub struct WindShift;
impl ParseForRemarks for WindShift {
    fn parse_rmk(info: &String) -> String {
        let sp = info.find(" ").expect("Unable to parse wind shift");
        if info.len() <= 8 {
            let min = &info[sp + 1..].parse::<u32>().expect("Unable to parse wind shift time");
            format!("Wind shift beginning {} minutes after the current hour", min)
        } else {
            let hour = &info[sp + 1..= sp + 2].parse::<u32>().expect("Unable to parse wind shift time");
            let min = &info[sp + 3..= sp + 4].parse::<u32>().expect("Unable to parse wind shift time");
            format!("Wind shift beginning at {}:{} UTC", hour, min)
        }
    }
}
mod tests {
    use crate::us_metar_components::*;
    #[test]
    fn visibility() {
        let a = Visibility::parse(&String::from("9SM"));
        let s1 = String::from("Visibility: 9 statute miles");
        assert_eq!(a, s1);
        let b = Visibility::parse(&String::from("M6SM"));
        let s2 = String::from("Visibility: less than 6 statute miles");
        assert_eq!(b, s2);
        let c = Visibility::parse(&String::from("P4SM"));
        let s3 = String::from("Visibility: more than 4 statute miles");
        assert_eq!(c, s3);
    }
    #[test]
    fn altimeter() {
        let a = Alt::parse(&String::from("A2973"));
        let s = String::from("Altimiter: 29.73 inHg\n");
        assert_eq!(a, s);
    }
    #[test]
    fn rvr() {
        let a = Rvr::parse(&String::from("R05L/1600FT"));
        let s1 = String::from("RVR for Runway 05L: 1600 ft");
        assert_eq!(a, s1);
        let b = Rvr::parse(&String::from("R27/1500V1700FT"));
        let s2 = String::from("RVR for Runway 27: Between 1500 ft and 1700 ft");
        assert_eq!(b, s2);
        let c = Rvr::parse(&String::from("R31/M1400VP1600FT"));
        let s3 = String::from("RVR for Runway 31: Between less than 1400 ft and more than 1600 ft");
        assert_eq!(c, s3);
    }
}