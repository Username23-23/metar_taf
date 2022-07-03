use crate::us_metar_components::*;
pub trait Name {
    fn name_needed(&self) -> String;
}
impl Name for When {
    fn name_needed(&self) -> String {
        let hour = (self.get_time() / 100);
        let minute = self.get_time() - (hour * 100);
        format!("Taken on the {}th day of the current month at {}:{} UTC\n", self.get_day(), hour, minute)
    }
} 
impl Name for Wind {
    fn name_needed(&self) -> String {
        let translate_dir = |a: &Range<u32>| -> String {
            if(a.is_empty()) {
                match a.start {
                    999 => String::from("variable"),
                    _ => format!("{} degrees", a.start)
                }
            } else {
                format!("varying between {} and {} degrees", a.start, a.end)
            }
        };
        let translate_spd = |a: &Range<u32>| -> String {
            if(a.is_empty()) {
                format!("{} knots", a.start)
            } else {
                format!("{} knots, with gusts of {} knots possible", a.start, a.end)
            }
        };
        format!("Wind direction: {}\nWind Speed: {}", translate_dir(&self.dir), translate_spd(&self.spd))
    }
}
impl Name for Visibility {
    fn name_needed(&self) -> String {
        match self {
            Visibility::Plus(a) => format!("More than {} statute miles\n", a),
            Visibility::Exact(b) => format!("{} statute miles\n", b),
            Visibility::Less(c) => format!("Less than {} statute miles\n", c),
            _ => String::new(),
        }
    }
}
impl Name for Cloud_layer {
    fn name_needed(&self) -> String {
        match self {
            Cloud_layer::Few(a) => format!("Few clouds at {} feet AGL\n", a),
            Cloud_layer::Sct(b) => format!("Scattered clouds at {} feet AGL\n", b),
            Cloud_layer::Bkn(c) => format!("Broken clouds at {} feet AGL\n", c),
            Cloud_layer::Ovc(d) => format!("Overcast clouds at {} feet AGL\n", d),
            Cloud_layer::ClrSkc(_) => format!("Clear skies"),
            _ => String::new(),
        }
    }    
}
impl Name for Alt {
    fn name_needed(&self) -> String {
        format!("Altimiter: {} inHg\n", self.get_alt_inhg())
    }    
}
impl Name for Temps {
    fn name_needed(&self) -> String {
        format!("Temperature: {} Celsius\nDewpoint: {} Celsius\n", self.get_temp(), self.get_dewpoint())
    }    
}
impl Name for Precip {
    fn name_needed(&self) -> String {
        match self {
            Precip::Dz(_) => format!("{} Drizzle\n", self.get_intensity()),
            Precip::Ra(_) => format!("{} Rain\n", self.get_intensity()),
            Precip::Tsra(_) => format!("{} Rain associated with thunderstorm\n", self.get_intensity()),
            Precip::Fzra(_) => format!("{} Freezing Rain\n", self.get_intensity()),
            Precip::Sn(_) => format!("{} Snow\n", self.get_intensity()),
            Precip::Sp(_) => format!("{} Snow Pellets\n", self.get_intensity()),
            Precip::Blsn(_) => format!("{} Blowing snow\n", self.get_intensity()),
        }
    }   
}
// extremely messy on borrowing; solution: look at wind
impl Name for Rvr {
    fn name_needed(&self) -> String {
        let handle_visib = |a: &Visibility| -> String {
            match *a {
                Visibility::Plus(a) => format!("more than {} ft", a),
                Visibility::Exact(b) => format!("{} ft", b),
                Visibility::Less(c) => format!("less than {} ft", c),
                _ => String::new(),
            }
        };
        if let Some(v) = self.get_upper_bound() {
            format!("RVR for Runway {}: between {} and {}", self.get_rwy(), handle_visib(self.get_vis()), handle_visib(&v))
        } else {
            format!("RVR for Runway {}: {}", self.get_rwy(), handle_visib(self.get_vis()))
        }
    }
}
mod tests {
    use crate::*;
    #[test]
    fn when() {
        let w = When::new(String::from("291314Z"));
        let s = String::from("Taken on the 29th day of the current month at 13:14 UTC\n");
        assert_eq!(w.name_needed(), s);
    }
    #[test]
    fn wind() {
        let a = Wind::new(String::from("08717G24KT"));
        let s1 = String::from("Wind direction: 87 degrees\nWind Speed: 17 knots, with gusts of 24 knots possible");
        assert_eq!(a.name_needed(), s1);
        let b = Wind::new(String::from("08717KT"));
        let s2 = String::from("Wind direction: 87 degrees\nWind Speed: 17 knots");
        assert_eq!(b.name_needed(), s2);
        let c = Wind::new(String::from("08717G24KT 086V088"));
        let s3 = String::from("Wind direction: varying between 86 and 88 degrees\nWind Speed: 17 knots, with gusts of 24 knots possible");
        assert_eq!(c.name_needed(), s3);
        let d = Wind::new(String::from("VRB03G05KT"));
        let s4 = String::from("Wind direction: variable\nWind Speed: 3 knots, with gusts of 5 knots possible");
        assert_eq!(d.name_needed(), s4);
    }
    #[test]
    fn visibility() {
        let a = Visibility::new(String::from("9SM"));
        let s1 = String::from("9 statute miles\n");
        assert_eq!(a.name_needed(), s1);
        let b = Visibility::new(String::from("M6SM"));
        let s2 = String::from("Less than 6 statute miles\n");
        assert_eq!(b.name_needed(), s2);
        let c = Visibility::new(String::from("P4SM"));
        let s3 = String::from("More than 4 statute miles\n");
        assert_eq!(c.name_needed(), s3);
    }
    #[test]
    fn cloud_layer() {
        let a = Cloud_layer::new(String::from("SCT036"));
        let s = String::from("Scattered clouds at 3600 feet AGL\n");
        assert_eq!(a.name_needed(), s);
    }
    #[test]
    fn altimeter() {
        let a = Alt::new(String::from("A2973"));
        let s = String::from("Altimiter: 29.73 inHg\n");
        assert_eq!(a.name_needed(), s);
    }
    #[test]
    fn rvr() {
        let a = Rvr::new(String::from("R05L/1600FT"));
        let s1 = String::from("RVR for Runway 05L: 1600 ft");
        assert_eq!(a.name_needed(), s1);
        let b = Rvr::new(String::from("R27/1500V1700FT"));
        let s2 = String::from("RVR for Runway 27: between 1500 ft and 1700 ft");
        assert_eq!(b.name_needed(), s2);
        let c = Rvr::new(String::from("R31/M1400VP1600FT"));
        let s3 = String::from("RVR for Runway 31: between less than 1400 ft and more than 1600 ft");
        assert_eq!(c.name_needed(), s3);
    }
    #[test]
    fn temps() {
        let a = Temps::new(String::from("17/14"));
        let s1 = String::from("Temperature: 17 Celsius\nDewpoint: 14 Celsius\n");
        assert_eq!(a.name_needed(), s1);
        let b = Temps::new(String::from("07/M03"));
        let s2 = String::from("Temperature: 7 Celsius\nDewpoint: -3 Celsius\n");
        assert_eq!(b.name_needed(), s2);
        let c = Temps::new(String::from("M09/M10"));
        let s3 = String::from("Temperature: -9 Celsius\nDewpoint: -10 Celsius\n");
        assert_eq!(c.name_needed(), s3);
    }
}