use crate::us_metar_components::*;
pub trait Name {
    fn name_needed(&self) -> String;
}
mod tests {
    use crate::*;
    //paused- these unit tests might be closer to what tests should look like after refactor
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