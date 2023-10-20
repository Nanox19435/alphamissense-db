use crate::aminoacids::{AminoAcid, AminoAcids};
use std::str::FromStr;

/// Clasificación de la variante de la proteína
pub enum AmClass {
    Benign(f64),
    Pathogenic(f64),
    Ambiguous(f64),
}

pub struct Variations {
    base: AminoAcid,
    variants: AminoAcids<AmClass>,
}

/// Representación de una variación
pub struct Variation {
    base: AminoAcid,
    position: u16,
    variant: AminoAcid,
}

impl FromStr for Variation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.len() - 1;

        let base = AminoAcid::from_str(&s[0..1])?;
        let position = s[1..i].parse::<u16>().map_err(|_| ())?;
        let variant = AminoAcid::from_str(&s[i..])?;

        return Ok(Variation {
            base,
            position,
            variant,
        });
    }
}