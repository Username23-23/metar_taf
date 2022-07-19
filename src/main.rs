pub mod us_metar_components;
pub mod world_metar;
pub mod parser;
use crate::parser::*;
fn main() {
    parse_metar("KTKI 071653Z 20007KT 10SM CLR 35/18 A2999 RMK AO2 SLP147 T03500178");
    parse_metar("KORD 071651Z 11004KT 10SM SCT032 SCT200 OVC250 28/19 A3002 RMK AO2 SLP163 T02830194");
    parse_metar("KSEA 071653Z 24006KT 10SM FEW025 BKN070 BKN120 BKN200 19/12 A3005 RMK AO2 SLP180 T01890122");
    parse_metar("KPIA 071654Z 00000KT 2SM VCTS RA BR BKN009 OVC016 23/22 A3004 RMK AO2 LTG DSNT S TSB30E45 SLP167 P0017 T02280222");
    parse_metar("KDFW 011242Z 35022G28KT 1SM R17C/5500VP6000FT -SN BR SCT009 BKN017 OVC027 M06/M07 A2988 RMK AO2 PK WND 33040/1202 SFC VIS 1 3/4 PLE16 PRESFR");
    parse_metar("KTKI 071653Z 20007KT 190V210 1/4SM VV028 35/18 A2999 RMK AO2 SLP147 T03500178");
    parse_metar("EGGW 112250Z AUTO 20005KT 9999 NCD 21/12 Q1023");
    parse_metar("EDDF 112250Z 04004KT 360V060 CAVOK 16/12 Q1024 NOSIG");
    parse_metar("RJTT 112230Z 10004KT 050V140 9999 FEW005 SCT010 BKN/// 25/24 Q1008 NOSIG RMK 1CU005 3CU010 A2978");
    parse_metar("NZAA 112300Z AUTO 33008KT 290V360 9999 SCT017/// BKN070/// 16/16 Q1000 TEMPO 6000 SHRA");
    parse_metar("YSSY 112300Z 31007KT 9999 FEW030 BKN220 11/08 Q1018");
}




