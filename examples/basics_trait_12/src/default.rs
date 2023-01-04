pub trait D {
    fn de(&self) -> String {
        String::from("default")
    }
    fn not_de() -> String;
}

pub struct P;

impl D for P {
    fn not_de() -> String {
        String::from("P")
    }
}

pub struct O;

impl D for O {
    fn de(&self) -> String {
        String::from("overwrited")
    }
    fn not_de() -> String {
        String::from("O")
    }
}
