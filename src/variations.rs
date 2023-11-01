use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::aminoacids::AminoAcid;

/// Clasificación de la variante de la proteína
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AmClass {
    Benign(f32),
    Pathogenic(f32),
    Ambiguous(f32),
    Undefined
}

impl ToString for AmClass {
    fn to_string(&self) -> String {
        match self {
            AmClass::Benign(w) => format!("Posiblemente benigno: {}", w),
            AmClass::Pathogenic(w) => format!("Posiblemente patógenico: {}", w),
            AmClass::Ambiguous(w) => format!("Ambiguo: {}", w),
            AmClass::Undefined => "No Definido".to_owned(),
        }
    }
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
