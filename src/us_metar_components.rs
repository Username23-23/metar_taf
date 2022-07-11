//TODO: cleanup- reduce repetitive calls, better on borrowing/refs, better error handling, format string slices
//This file contains items needed to parse encoded groups that are found only in US METAR repots
use crate::world_metar::Parser;
pub struct USVisibility;
impl Parser for USVisibility {
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
impl Parser for USCloudLayer {
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
impl Parser for Alt {
    fn parse(info: &String) -> String {
        let alt = (info[1..].parse::<f64>().expect("Couldn't parse altimeter setting")) / 100.0;
        format!("Altimiter: {} inHg\n", alt)
    }
}
pub struct USRvr;
impl Parser for USRvr {
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