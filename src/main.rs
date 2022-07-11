pub mod us_metar_components;
pub mod better_name;
pub mod world_metar;
use crate::us_metar_components::*;
use crate::world_metar::*;
//TODO: deal with ownership/memory better, RMK section, think about SPECI and amendments, parse as string slices
fn parse_us(met: &str)  {
    let metar = String::from(met);
    let s = metar.split_whitespace(); //mixed frac for visibility
    let mut a: Vec<String> = Vec::new();
    'outer: for s1 in s {
        if(s1 == "METAR") {
            println!("");
        } else if(s1 == "AUTO") {
            println!("The METAR that follows is an automated observation");
        } else if(s1 == "RMK") {
            break 'outer;
        } else {
            a.push(String::from(s1));
        }
    }
    let num_variable_components = a.len() - 5;
    println!("Station: {}", a.get(0).unwrap());
    println!("{}", When::parse(a.get(1).unwrap()));
    println!("{}", Wind::parse(a.get(2).unwrap()));
    let mut curr_index = 3;
    for i in (0..num_variable_components) {
        //TODO: where might these checks fail-> reorder to make it less likely to fail, but also efficient
        let element = a.get(curr_index).unwrap();
        let first = &element[0..1];
        let l = element.len();
        if(l > 7) {
            println!("{}", USRvr::parse(element));
        } else if(l == 6 || first == "S" || first == "C") {
            println!("{}", USCloudLayer::parse(element));
        } else if(l >= 5 && &element[3..4] == "V") {
            println!("{}", VariableWindDirection::parse(element));
        } else if(first == "V" && l == 5) {
            //VCTS might fail this
            println!("{}", VerticalVisibility::parse(element));
        } else if(&element[l - 1..] == "M") {
            println!("{}", USVisibility::parse(element));
        } else {
            println!("{}", Weather::parse(element));
        }
        curr_index += 1;
    }
    println!("{}", Temps::parse(a.get(curr_index).unwrap()));
    curr_index += 1;
    println!("{}\n\n\n", Alt::parse(a.get(curr_index).unwrap()));
}
fn parse_world(met: &str) {
    let metar = String::from(met);
    let s = metar.split_whitespace(); 
    let mut a: Vec<String> = Vec::new();
    'outer: for s1 in s {
        if(s1 == "METAR") {
            println!("");
        } else if(s1 == "AUTO") {
            println!("The METAR that follows is an automated observation");
        } else if(s1 == "CAVOK") {
            println!("No clouds below 5000 ft, no significant weather phenomena, and visibility at least 10 km");
        } else if(&s1[0..1] == "Q") {
            a.push(String::from(s1));
            break 'outer;
        } else {
            a.push(String::from(s1));
        }
    }
    let num_var = a.len() - 5;
    println!("Station: {}", a.get(0).unwrap());
    println!("{}", When::parse(a.get(1).unwrap()));
    println!("{}", Wind::parse(a.get(2).unwrap()));
    let mut curr_index = 3;
    for i in (0..num_var) {
        let element = a.get(curr_index).unwrap();
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
    println!("{}", Temps::parse(a.get(curr_index).unwrap()));
    curr_index += 1;
    println!("{}\n\n\n", Qnh::parse(a.get(curr_index).unwrap()));
}
fn main() {
    parse_us("KTKI 071653Z 20007KT 10SM CLR 35/18 A2999 RMK AO2 SLP147 T03500178");
    parse_us("KORD 071651Z 11004KT 10SM SCT032 SCT200 OVC250 28/19 A3002 RMK AO2 SLP163 T02830194");
    parse_us("KSEA 071653Z 24006KT 10SM FEW025 BKN070 BKN120 BKN200 19/12 A3005 RMK AO2 SLP180 T01890122");
    parse_us("KPIA 071654Z 00000KT 2SM VCTS RA BR BKN009 OVC016 23/22 A3004 RMK AO2 LTG DSNT S TSB30E45 SLP167 P0017 T02280222");
    parse_us("KDFW 011242Z 35022G28KT 1SM R17C/5500VP6000FT -SN BR SCT009 BKN017 OVC027 M06/M07 A2988 RMK AO2 PK WND 33040/1202 SFC VIS 1 3/4 PLE16 PRESFR");
    parse_us("KTKI 071653Z 20007KT 190V210 1/4SM VV028 35/18 A2999 RMK AO2 SLP147 T03500178");
    parse_world("EGGW 112250Z AUTO 20005KT 9999 NCD 21/12 Q1023");
    parse_world("EDDF 112250Z 04004KT 360V060 CAVOK 16/12 Q1024 NOSIG");
    parse_world("RJTT 112230Z 10004KT 050V140 9999 FEW005 SCT010 BKN/// 25/24 Q1008 NOSIG RMK 1CU005 3CU010 A2978");
    parse_world("NZAA 112300Z AUTO 33008KT 290V360 9999 SCT017/// BKN070/// 16/16 Q1000 TEMPO 6000 SHRA");
    parse_world("YSSY 112300Z 31007KT 9999 FEW030 BKN220 11/08 Q1018");
}




