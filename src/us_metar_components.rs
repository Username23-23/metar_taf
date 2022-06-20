//TODO: FINISH UNIT TESTS AND MOVE TRAIT TO SEPARATE FILE, PVT FIELDS & GET/SET
pub struct When {
    pub day_of_month: i32,
    pub zulu_time: i32, 
}

impl When {
    pub fn new(info: String) -> Self {
        Self {
            day_of_month: info[..2].parse::<i32>().unwrap(),
            zulu_time: info[2..6].parse::<i32>().unwrap(),
        }
    }
}

pub struct Wind {
    pub dir: i32,
    pub spd: i32,
    pub gust: Option<i32>,
}

impl Wind {
    pub fn new(info: String) -> Self {
        if info.len() <= 7 {
            Self {
                dir: info[..3].parse::<i32>().unwrap(),
                spd: info[3..5].parse::<i32>().unwrap(),
                gust: None, 
            }
        } else {
            Self {
                dir: info[..3].parse::<i32>().unwrap(),
                spd: info[3..5].parse::<i32>().unwrap(),
                gust: Some(info[6..8].parse::<i32>().unwrap()),
            }
        }
    }
}
//TODO: figure out rvr and all that stuff, FIX THE UNWRAP MESS
pub enum Visibility {
    Plus(i32),
    Exact(i32), 
    Less(i32), //dubious
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
}
pub enum Cloud_layer {
    few(i32),
    sct(i32),
    bkn(i32),
    ovc(i32),
    clr_skc(i32),
}
//dummy number for clr_skc
impl Cloud_layer {
    pub fn new(info: String) -> Self {
        match &info[0..3] {
            "OVC" => Cloud_layer::ovc(info[3..6].parse::<i32>().unwrap() * 100),
            "BKN" => Cloud_layer::bkn(info[3..6].parse::<i32>().unwrap() * 100),
            "SCT" => Cloud_layer::sct(info[3..6].parse::<i32>().unwrap() * 100),
            "FEW" => Cloud_layer::few(info[3..6].parse::<i32>().unwrap() * 100),
            _ => Cloud_layer::clr_skc(0),
        }
    }
} 

pub struct Alt {
   pub alt_inhg: f64,
}
impl Alt { 
    pub fn new(info: String) -> Self {
        Self {
            alt_inhg: (info[1..].parse::<f64>().unwrap()) / 100.0,
        }
    }
}
pub struct Temps {
    pub temp_celsius: i32,
    pub dewpoint_celsius: i32
}

impl Temps {
    pub fn new(info: String) -> Self {
        match info.len() {
            5 => {
                Self {
                    temp_celsius: info[..2].parse::<i32>().unwrap(),
                    dewpoint_celsius: info[3..].parse::<i32>().unwrap(),
                }
            },
            6 => {  
                //bc temp cant be neg w pos dewpoint 
                Self {
                    temp_celsius: info[0..2].parse::<i32>().unwrap(),
                    dewpoint_celsius: info[4..].parse::<i32>().unwrap() - (2 * info[4..].parse::<i32>().unwrap()),
                }
            },
            7 => {
                Self {
                    temp_celsius: info[1..3].parse::<i32>().unwrap() - (2 * info[1..3].parse::<i32>().unwrap()),
                    dewpoint_celsius: info[5..].parse::<i32>().unwrap() - (2 * info[5..].parse::<i32>().unwrap()),
                }
            },
            _ => panic!("AAAA")
        }
    }
}
//needs a LOT of work - BR Missing
pub enum Precip {
    Ra(i32),
    Dz(i32),
    Fzra(i32),
    Tsra(i32),
    Sn(i32),
    Sp(i32),
    Blsn(i32),
}
impl Precip {
    pub fn new(info: String) -> Self {
        if(&info[0..1] == "+") {
            match &info[1..] {
                "DZ" => Precip::Dz(1),
                "FZRA" => Precip::Fzra(1),
                "TSRA" => Precip::Tsra(1),
                "SN" => Precip::Sn(1),
                "SP" => Precip::Sp(1),
                "BLSN" => Precip::Blsn(1),
                _ => Precip::Ra(1),
            }
        } else if(&info[0..1] == "-") {
            match &info[1..] {
                "DZ" => Precip::Dz(-1),
                "FZRA" => Precip::Fzra(-1),
                "TSRA" => Precip::Tsra(-1),
                "SN" => Precip::Sn(-1),
                "SP" => Precip::Sp(-1),
                "BLSN" => Precip::Blsn(-1),
                _ => Precip::Ra(-1),
            }
        } else {
            match &info[0..] {
                "DZ" => Precip::Dz(0),
                "FZRA" => Precip::Fzra(0),
                "TSRA" => Precip::Tsra(0),
                "SN" => Precip::Sn(0),
                "SP" => Precip::Sp(0),
                "BLSN" => Precip::Blsn(0),
                _ => Precip::Ra(0),
            }
        }
    }
    // MAKE THIS LESS HACKY AND MORE RUST-like store intensity as string in enum
    pub fn get_intensity(&self) -> String {
        let translate = |num: i32| -> String {
            match num {
                -1 => String::from("Light"),
                0 => String::from("Moderate"),
                1 => String::from("Heavy"),
                _ => String::new(),
            }
        };
        match self {
            Precip::Dz(a) => translate(*a),
            Precip::Fzra(b) => translate(*b),
            Precip::Tsra(c) => translate(*c),
            Precip::Sn(d) => translate(*d),
            Precip::Sp(e) => translate(*e),
            Precip::Blsn(f) => translate(*f),
            Precip::Ra(g) => translate(*g),
        }
    }
}
mod tests {
    use crate::us_metar_components::*;
    #[test]
    fn check_when() {
        let w = When::new(String::from("291314Z"));
        assert_eq!(w.day_of_month, 29);
        assert_eq!(w.zulu_time, 1314);
    }
    #[test]
    fn check_wind() {
        let w = Wind::new(String::from("08717G24KT"));
        assert_eq!(w.dir, 087);
        assert_eq!(w.spd, 17);
        assert_eq!(w.gust, Some(24));
        let no = Wind::new(String::from("08717KT"));
        assert_eq!(no.dir, 087);
        assert_eq!(no.spd, 17);
        assert_eq!(no.gust, None);
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
    fn check_precip() {
        let a = Precip::new(String::from("+RA"));
        let b = Precip::new(String::from("BLSN"));
        let c = Precip::new(String::from("-FZRA"));
        assert_eq!(a.get_intensity(), 1);
        assert_eq!(b.get_intensity(), 0);
        assert_eq!(c.get_intensity(), -1);
    }
    #[test]
    fn check_visibility() {
        //REMINDER THAT RVR ISNT IMPLEMENTED ONLY REGULAR VISIB
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
    fn check_clouds() {
        let a = Cloud_layer::new(String::from("SCT036"));
        assert_eq!({
            match a {
                Cloud_layer::sct(b) => b,
                _ => 0
            }
        }, 3600);
    }
}