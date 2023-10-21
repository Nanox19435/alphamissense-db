use std::str::FromStr;

pub enum AminoAcid {
    Alanine,
    Arginine,
    Asparagine,
    AsparticAcid,
    Cysteine,
    GlutamicAcid,
    Glutamine,
    Glycine,
    Histidine,
    Isoleucine,
    Leucine,
    Lysine,
    Methionine,
    Phenylalanine,
    Proline,
    Serine,
    Threonine,
    Tryptophan,
    Tyrosine,
    Valine,
}



impl AminoAcid {
    pub const SINGLE_LETTER_CODE: AminoAcidMap<char> = AminoAcidMap([
        'A', 'R', 'N', 'D', 'C', 'E', 'Q', 'G', 'H', 'I', 'L', 'K', 'M', 'F', 'P', 'S', 'T', 'W',
        'Y', 'V',
    ]);

    pub const THREE_LETTER_CODE: AminoAcidMap<&'static str> = AminoAcidMap([
        "ALA", "ARG", "ASN", "ASP", "CYS", "GLU", "GLN", "GLY", "HIS", "ILE", "LEU", "LYS", "MET",
        "PHE", "PRO", "SER", "THR", "TRP", "TYR", "VAL",
    ]);

    pub(crate) fn index(&self) -> usize {
        match self {
            AminoAcid::Alanine => 0,
            AminoAcid::Arginine => 1,
            AminoAcid::Asparagine => 2,
            AminoAcid::AsparticAcid => 3,
            AminoAcid::Cysteine => 4,
            AminoAcid::GlutamicAcid => 5,
            AminoAcid::Glutamine => 6,
            AminoAcid::Glycine => 7,
            AminoAcid::Histidine => 8,
            AminoAcid::Isoleucine => 9,
            AminoAcid::Leucine => 10,
            AminoAcid::Lysine => 11,
            AminoAcid::Methionine => 12,
            AminoAcid::Phenylalanine => 13,
            AminoAcid::Proline => 14,
            AminoAcid::Serine => 15,
            AminoAcid::Threonine => 16,
            AminoAcid::Tryptophan => 17,
            AminoAcid::Tyrosine => 18,
            AminoAcid::Valine => 19,
        }
    }
}

impl FromStr for AminoAcid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "Ala" | "A" => Ok(AminoAcid::Alanine),
            "Arg" | "R" => Ok(AminoAcid::Arginine),
            "Asn" | "N" => Ok(AminoAcid::Asparagine),
            "Asp" | "D" => Ok(AminoAcid::AsparticAcid),
            "Cys" | "C" => Ok(AminoAcid::Cysteine),
            "Glu" | "E" => Ok(AminoAcid::GlutamicAcid),
            "Gln" | "Q" => Ok(AminoAcid::Glutamine),
            "Gly" | "G" => Ok(AminoAcid::Glycine),
            "His" | "H" => Ok(AminoAcid::Histidine),
            "Ile" | "I" => Ok(AminoAcid::Isoleucine),
            "Leu" | "L" => Ok(AminoAcid::Leucine),
            "Lys" | "K" => Ok(AminoAcid::Lysine),
            "Met" | "M" => Ok(AminoAcid::Methionine),
            "Phe" | "F" => Ok(AminoAcid::Phenylalanine),
            "Pro" | "P" => Ok(AminoAcid::Proline),
            "Ser" | "S" => Ok(AminoAcid::Serine),
            "Thr" | "T" => Ok(AminoAcid::Threonine),
            "Trp" | "W" => Ok(AminoAcid::Tryptophan),
            "Tyr" | "Y" => Ok(AminoAcid::Tyrosine),
            "Val" | "V" => Ok(AminoAcid::Valine),
            _ => Err(()), // Invalid input
        }
    }
}

/// Struct que permite relacional los 20 aminoacidos con cualquier tipo de datos.
/// TODO: Mejorar la API, probablemente armando a través de un iterador.
pub struct AminoAcidMap<T>(pub [T; 20]);

impl<T> std::ops::Index<AminoAcid> for AminoAcidMap<T> {
    type Output = T;

    fn index(&self, index: AminoAcid) -> &Self::Output {
        let AminoAcidMap(variants) = self;

        variants.index(index.index())
    }
}

impl<T> std::ops::IndexMut<AminoAcid> for AminoAcidMap<T> {
    fn index_mut(&mut self, index: AminoAcid) -> &mut Self::Output {
        let AminoAcidMap(variants) = self;

        variants.index_mut(index.index())
    }
}