use crate::us_metar_components::*;
pub trait Name {
    fn name_needed(&self) -> String;
}
impl Name for When {
    fn name_needed(&self) -> String {
        let hour = (self.zulu_time / 100);
        let minute = self.zulu_time - (hour * 100);
        format!("Taken on the {}th day of the current month at {}:{} UTC\n", self.day_of_month.to_string(), hour.to_string(), minute.to_string())
    }
} 
impl Name for Wind {
    fn name_needed(&self) -> String {
        if let Some(add) = self.gust {
            format!("Wind blowing from {} degrees at {} knots, with gusts of an additional {} knots \n", self.dir.to_string(), self.spd.to_string(), add)
        } else {
            format!("Wind blowing from {} degrees at {} knots\n", self.dir.to_string(), self.spd.to_string())
        }
    }
}
impl Name for Visibility {
    fn name_needed(&self) -> String {
        match self {
            Visibility::Plus(a) => format!("More than {} statute miles\n", a.to_string()),
            Visibility::Exact(b) => format!("{} statute miles\n", b.to_string()),
            Visibility::Less(c) => format!("Less than {} statute miles\n", c.to_string()),
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
        format!("Altimiter: {} inHg\n", self.alt_inhg.to_string())
    }    
}
impl Name for Temps {
    fn name_needed(&self) -> String {
        format!("Temperature: {} Celsius\nDewpoint: {} Celsius\n", self.temp_celsius.to_string(), self.dewpoint_celsius.to_string())
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
mod tests {
    
}