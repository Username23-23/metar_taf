pub trait Name {
    fn name_needed(&self) -> String;
}
//leave this here in case there is a need to switch back to old approach (reading data into structs/enums)