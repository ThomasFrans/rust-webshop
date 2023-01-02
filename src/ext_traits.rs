use diesel::data_types::PgNumeric;
use std::num::ParseFloatError;

pub trait PgNumericExt {
    fn try_from_float(weight: i16, scale: u16, float: f32) -> Result<PgNumeric, ParseFloatError>;
}

impl PgNumericExt for PgNumeric {
    fn try_from_float(weight: i16, scale: u16, float: f32) -> Result<PgNumeric, ParseFloatError> {
        todo!()
    }
}
