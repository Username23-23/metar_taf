//TODO: deal with ownership/memory better, RMK section, think about SPECI and amendments, parse as string slices
use crate::us_metar_components::*;
use crate::world_metar::*;
use crate::taf_only_groups::*;
pub fn parse_metar(met: &str) {
    if(&met[0..=4] == "METAR") {
        match &met[6..7] {
            "K" | "P" => parse_us(met),
            "C" => parse_other_north_american(met),
            "M" => {
                match &met[7..8] {
                    "M" => parse_other_north_american(met),
                    _ => parse_world(met),
                }
            },
            _ => parse_world(met),
        }
    } else {
        match &met[0..1] {
            "K" => parse_us(met),
            "C" => parse_other_north_american(met),
            "M" => {
                match &met[1..2] {
                    "M" => parse_other_north_american(met),
                    _ => parse_world(met),
                }
            },
            _ => parse_world(met),
        }
    }
}
fn parse_us(met: &str)  {
    let metar = String::from(met);
    let s = metar.split_whitespace(); 
    let mut primary: Vec<String> = Vec::new();
    let mut rmks: Vec<String> = Vec::new();
    let mut at_rmk = false;
    for s1 in s {
        if(s1 == "METAR") {
            println!("");
        } else if(s1 == "AUTO") {
            println!("\n\nThe METAR that follows is an automated observation");
        } else if(s1 == "RMK") {
            at_rmk = true;
        } else if(at_rmk) {
            rmks.push(String::from(s1));
        } else {
            primary.push(String::from(s1));
        }
    }
    let num_variable_components = primary.len() - 5;
    println!("\nStation: {}", primary.get(0).unwrap());
    println!("{}", When::parse(primary.get(1).unwrap()));
    println!("{}", Wind::parse(primary.get(2).unwrap()));
    let mut curr_index = 3;
    for i in (0..num_variable_components) {
        //TODO: where might these checks fail-> reorder to make it less likely to fail, but also efficient
        let element = primary.get(curr_index).unwrap();
        let first = &element[0..1];
        let l = element.len();
        if(l > 7) {
            println!("{}", USRvr::parse(element));
        } else if(l == 6 || first == "S" || first == "C") {
            println!("{}", USCloudLayer::parse(element));
        } else if(l >= 5 && &element[3..4] == "V") {
            println!("{}", VariableWindDirection::parse(element));
        } else if(first == "V" && l == 5) {
            println!("{}", VerticalVisibility::parse(element));
        } else if(&element[l - 1..] == "M") {
            println!("{}", USVisibility::parse(element));
        } else {
            println!("{}", Weather::parse(element));
        }
        curr_index += 1;
    }
    println!("{}", Temps::parse(primary.get(curr_index).unwrap()));
    curr_index += 1;
    println!("{}", Alt::parse(primary.get(curr_index).unwrap()));
    let mut i = 0;
    while(i < rmks.len()) {
        let el = rmks.get(i).unwrap();
        let first = &el[0..1];
        let l = el.len();
        if(first == "A" && l == 3) {
            println!("{}", SensorType::parse_rmk(el));
        } else if(first == "S" && l == 6) {
            println!("{}", SeaLevelPressure::parse_rmk(el));
        } else if(first == "T" && l == 9) {
            println!("{}", AdditionalTemperatureData::parse_rmk(el));
        } else if(first == "P" && l == 2) {
            println!("{}", PeakWind::parse_rmk(&format!("{} {} {}", el, rmks.get(i + 1).unwrap(), rmks.get(i + 2).unwrap())));
            i += 2;
        } else if(first == "W" && l == 5) {
            println!("{}", WindShift::parse_rmk((&format!("{} {}", el, rmks.get(i + 1).unwrap()))));
            i += 1;
        }
        i += 1;
    }
}
fn parse_other_north_american(met: &str) {
    let metar = String::from(met);
    let s = metar.split_whitespace(); 
    let mut primary: Vec<String> = Vec::new();
    'outer: for s1 in s {
        if(s1 == "METAR") {
            println!("");
        } else if(s1 == "AUTO") {
            println!("\n\nThe METAR that follows is an automated observation");
        } else if(s1 == "RMK") {
            break 'outer;
        } else {
            primary.push(String::from(s1));
        }
    }
    let num_variable_components = primary.len() - 5;
    println!("\nStation: {}", primary.get(0).unwrap());
    println!("{}", When::parse(primary.get(1).unwrap()));
    println!("{}", Wind::parse(primary.get(2).unwrap()));
    let mut curr_index = 3;
    for i in (0..num_variable_components) {
        //TODO: where might these checks fail-> reorder to make it less likely to fail, but also efficient
        let element = primary.get(curr_index).unwrap();
        let first = &element[0..1];
        let l = element.len();
        if(l > 7) {
            println!("{}", USRvr::parse(element));
        } else if(l == 6 || first == "S" || first == "C") {
            println!("{}", USCloudLayer::parse(element));
        } else if(l >= 5 && &element[3..4] == "V") {
            println!("{}", VariableWindDirection::parse(element));
        } else if(first == "V" && l == 5) {
            println!("{}", VerticalVisibility::parse(element));
        } else if(&element[l - 1..] == "M") {
            println!("{}", USVisibility::parse(element));
        } else {
            println!("{}", Weather::parse(element));
        }
        curr_index += 1;
    }
    println!("{}", Temps::parse(primary.get(curr_index).unwrap()));
    curr_index += 1;
    println!("{}", Alt::parse(primary.get(curr_index).unwrap()));
}
fn parse_world(met: &str) {
    let metar = String::from(met);
    let s = metar.split_whitespace(); 
    let mut primary: Vec<String> = Vec::new();
    let mut extra: Vec<String> = Vec::new();
    let mut forecast: Vec<String> = Vec::new(); //TODO: parse trend forecast
    let mut extra_reached = false;
    let mut forecast_reached = false;
    'outer: for s1 in s {
        if s1 == "METAR" {
            print!("");
        } else if s1 == "AUTO" {
            println!("\n\nThe METAR that follows is an automated observation");
        } else if s1 == "CAVOK" {
            println!("No clouds below 5000 ft, no significant weather phenomena, and visibility at least 10 km");
        } else if &s1[0..1] == "Q" {
            primary.push(String::from(s1));
            extra_reached = true;
        } else if s1 == "TEMPO" || s1 == "NOSIG" || s1 == "BECMG" {
            forecast_reached = true;
            forecast.push(String::from(s1));
        } else if extra_reached && forecast_reached {
            forecast.push(String::from(s1));
        } else if extra_reached {
            extra.push(String::from(s1));
        } else if s1 == "RMK" {
            break 'outer;
        } else {
            primary.push(String::from(s1));
        }
    }
    let num_var = primary.len() - 5;
    println!("\nStation: {}", primary.get(0).unwrap());
    println!("{}", When::parse(primary.get(1).unwrap()));
    println!("{}", Wind::parse(primary.get(2).unwrap()));
    let mut curr_index = 3;
    for i in (0..num_var) {
        let element = primary.get(curr_index).unwrap();
        let first = &element[0..1];
        let l = element.len();
        if(first == "R" && l > 8) {
            println!("{}", Rvr::parse(element));
        } else if(first == "V") {
            println!("{}", VerticalVisibility::parse(element));
        } else if(l == 7 && &element[3..4] == "V") {
            println!("{}", VariableWindDirection::parse(element));
        } else if let (4, Ok(_)) = (l, first.parse::<u32>()) {
            println!("{}", Visibility::parse(element));
        } else if let (5, Ok(_)) = (l, first.parse::<u32>()) {
            println!("{}", DirectionalVisibility::parse(element));
        } else if((first == "N" && l == 3) || (l == 6 || l == 9)) {
            println!("{}", CloudLayer::parse(element));
        } else {
            println!("{}", Weather::parse(element));
        }
        curr_index += 1;
    }
    println!("{}", Temps::parse(primary.get(curr_index).unwrap()));
    curr_index += 1;
    println!("{}\n\n\n", Qnh::parse(primary.get(curr_index).unwrap()));
    let mut e_i = 0;
    while(e_i < extra.len()) {
        let element = extra.get(e_i).unwrap();
        let first_two = &element[0..2];
        if first_two == "RE" {
            println!("{}", RecentWeather::parse(element));
        } else if first_two == "WS" {
            let next = extra.get(e_i + 1).unwrap();
            if(&next[0..1] == "A") {
                println!("{}", WindshearInformation::parse(&format!("{} {} {}", element, next, extra.get(e_i + 2).unwrap())));
                e_i += 2;
            } else {
                println!("{}", WindshearInformation::parse(&format!("{} {}", element, next)));
                e_i += 1;
            }
        } else if &element[0..1] == "R" {
            println!("{}", RunwayState::parse(element));
        } else {
            println!("{}", SeaInfo::parse(element));
        }
        e_i += 1;
    }
}
pub fn parse_us_taf(t: &str) {
    let mut primary: Vec<String> = Vec::new();    
    let l = t.split_whitespace();
    for s in l { 
        if s == "TAF" || s == "AMD" || s == "COR" {
            println!("{}", s);
        } else if s == "RMK" {
            break;
        } else {
            primary.push(String::from(s));
        }
    }
    println!("Station: {}", primary.get(0).unwrap());
    println!("{}", When::parse(primary.get(1).unwrap()));
    println!("{}", ValidityPeriod::parse(primary.get(2).unwrap()));
    let mut c_i = 3;
    while(c_i < primary.len()) {
        let element = primary.get(c_i).unwrap();
        let first = &element[0..1];
        let l = element.len();
        if l == 6 || first == "S" || first == "C" {
            println!("{}", USCloudLayer::parse(element));
        } else if first == "V" && l == 5 {
            println!("{}", VerticalVisibility::parse(element));
        } else if l >= 7 && &element[l - 1..] == "T" {
            println!("{}", Wind::parse(element));
        } else if &element[0..=1] == "WS" {
            println!("{}", LowLevelWindshear::parse(element));
        } else if l == 9 && &element[4..5] == "/" {
            println!("{}", ValidityPeriod::parse(element));
        } else if(l == 5 && (first == "B" || first == "T")) || (first == "P" && l == 6) || (first == "F" && l > 6) {
            println!("{}", ChangeIndicators::parse(element));
        } else if l >= 3 && &element[l - 2..] == "SM" {
            println!("{}", USVisibility::parse(element));
        } else {
            println!("{}", Weather::parse(element));
        }
        c_i += 1;
    }
}
