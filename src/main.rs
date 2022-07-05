pub mod us_metar_components;
pub mod other_metar;
pub mod better_name;
use crate::better_name::*;
use crate::us_metar_components::*;
//THINK ABOUT OWNERSHIP
//doesnt deal w any weather yet
fn parse(metar: String)  {
    /*
    let mut s = metar.split_whitespace();
    let mut a: Vec<String> = Vec::new();
    for s1 in s {
        a.push(String::from(s1));
    }
    let variable_comp_num = a.len() - 6;
    //for now, num clouds, should expand in future
    let time = When::new(a.get(1).unwrap().clone());
    let wind = Wind::new(a.get(2).unwrap().clone());
    let vis = Visibility::new(a.get(3).unwrap().clone());
    let mut cl: Vec<Cloud_layer> = Vec::new();
    let mut i = 4;
    while(i < 4 + variable_comp_num) {
        cl.push(Cloud_layer::new(a.get(i).unwrap().clone()));
        i += 1;
    }
    let t = Temps::new(a.get(i).unwrap().clone());
    i += 1;
    let alt = Alt::new(a.get(i).unwrap().clone());
    println!("{}", time.name_needed());
    println!("{}", wind.name_needed());
    println!("{}", vis.name_needed());
    for qw in &cl {
        println!("{}", qw.name_needed());
    }
    println!("{}", t.name_needed());
    println!("{}", alt.name_needed());
    */
}
fn main() {
    let mettt = String::from("KDFW 201253Z 13008KT 10SM BKN073 OVC090 FEW250 26/19 A3008");
    parse(mettt);
}




