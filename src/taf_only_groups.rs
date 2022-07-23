use crate::world_metar::*;
pub struct ValidityPeriod;
impl EncodedGroup for ValidityPeriod {
    fn parse(raw_data: &String) -> String {
        let start_day = &raw_data[0..=1].parse::<u32>().expect("Unable to parse forecast validity period - start day");
        let start_hour = &raw_data[2..=3].parse::<u32>().expect("Unable to parse forecast validity period - start hour");
        let end_day = &raw_data[5..=6].parse::<u32>().expect("Unable to parse forecast validity period - start hour");
        let end_hour = &raw_data[7..=8].parse::<u32>().expect("Unable to parse forecast validity period - start hour");
        format!("(Valid From {}:00 UTC on day {} of the current month to {}:00 UTC on day {} of the current month)", start_hour, start_day, end_hour, end_day)
    }
}
pub struct ChangeIndicators;
impl EncodedGroup for ChangeIndicators {
    fn parse(raw_data: &String) -> String {
        match &raw_data[0..1] {
            "F" => {
                let m = raw_data.find("M").expect("Error parsing change indicator FM");
                let day = &raw_data[m + 1..=m + 2].parse::<u32>().expect("Error parsing change indicator FM (day)");
                let hour = &raw_data[m + 3..=m + 4].parse::<u32>().expect("Error parsing change indicator FM (hour)");
                let minute = &raw_data[m + 5..=m + 6].parse::<u32>().expect("Error parsing change indicator FM (minute)");
                format!("Forecast to change to (starting from {}:{} UTC on day {} of the current month): ", hour, minute, day)
            },
            "T" => String::from("Forecast to temporarily change to"),
            "P" => {
                let b = raw_data.find("B").expect("Error parsing change indicator PROB");
                let percentage_chance = &raw_data[b + 1..].parse::<u32>().expect("Error parsing probablity in change indicator PROB");
                format!("{}% chance of becoming ", percentage_chance)
            },
            "B" => String::from("Becoming"),
            _ => String::from("Unknown change indicator"),
        }
    }
}
pub struct LowLevelWindshear;
impl EncodedGroup for LowLevelWindshear {
    fn parse(raw_data: &String) -> String {
        let hgt = &raw_data[2..=4].parse::<u32>().expect("Unable to parse windshear group");
        format!("Windshear at {} ft AGL ({})", hgt, Wind::parse(&String::from(&raw_data[6..])))
    }
}