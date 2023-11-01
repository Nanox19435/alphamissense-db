use serde::{Serialize, Deserialize};

use crate::aminoacids::AminoAcid;
use std::str::FromStr;

/// Clasificación de la variante de la proteína
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AmClass {
    Benign(f32),
    Pathogenic(f32),
    Ambiguous(f32),
    Undefined
}

/// Representación de una variación
pub struct Variation {
    /// Base del aminoacido.
    pub base: AminoAcid,
    /// Posicion de la variación.
    pub position: u16,
    /// Aminoacido modificado.
    pub variant: AminoAcid,
}

impl FromStr for Variation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 {
            return Err(());
        } 

        let i = s.len() - 1;

        let base = AminoAcid::from_str(&s[0..1])?;
        let position = s[1..i].parse::<u16>().map_err(|_| ())?;
        let variant = AminoAcid::from_str(&s[i..])?;

        Ok(Variation {
            base,
            position,
            variant,
        })
    }
}
