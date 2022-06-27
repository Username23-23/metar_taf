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
        if let Some(add) = self.get_gust() {
            format!("Wind blowing from {} degrees at {} knots, with gusts of an additional {} knots \n", self.get_dir(), self.get_spd(), add)
        } else {
            format!("Wind blowing from {} degrees at {} knots\n", self.get_dir(), self.get_spd())
        }
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
            Cloud_layer::few(a) => format!("Few clouds at {} feet AGL\n", a),
            Cloud_layer::sct(b) => format!("Scattered clouds at {} feet AGL\n", b),
            Cloud_layer::bkn(c) => format!("Broken clouds at {} feet AGL\n", c),
            Cloud_layer::ovc(d) => format!("Overcast clouds at {} feet AGL\n", d),
            Cloud_layer::clr_skc(_) => format!("Clear skies"),
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
// extremely messy on borrowing
impl Name for Rvr {
    fn name_needed(&self) -> String {
        let handle_visib = |a: &Visibility| -> String {
            match *a {
                Visibility::Plus(a) => format!("More than {} ft", a),
                Visibility::Exact(b) => format!("{} ft", b),
                Visibility::Less(c) => format!("Less than {} ft", c),
                _ => String::new(),
            }
        };
        if let Some(v) = self.get_upper_bound() {
            format!("RVR for Runway {}: between {} and {} ft", self.get_rwy(), handle_visib(self.get_vis()), handle_visib(&v))
        } else {
            format!("RVR for Runway {}: {} ft", self.get_rwy(), handle_visib(self.get_vis()))
        }
    }
}
mod tests {
    //STILL WAITING
}