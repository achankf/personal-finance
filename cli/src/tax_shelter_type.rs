#[derive(Debug, clap::ValueEnum, Clone)]
pub enum TaxShelterTypeValue {
    TFSA,
    FHSA,
    RRSP,
}

impl std::fmt::Display for TaxShelterTypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            TaxShelterTypeValue::TFSA => "TFSA",
            TaxShelterTypeValue::FHSA => "FHSA",
            TaxShelterTypeValue::RRSP => "RRSP",
        };

        write!(f, "{}", str)
    }
}
