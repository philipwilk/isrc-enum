#[derive(Debug, PartialEq)]
pub enum Isrc {
    Code(String),
}

impl TryFrom<String> for Isrc {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Additional country codes for when the US got too big
        let overflow_ccs: Vec<&'static str> = vec![
            "QM",
            "CP",
            "DG",
            "ZZ",
            "QZ",
            "GX",
            "UK",
            "KS",
            "us",
            "BC"
        ];

        // Remove optional hyphens from code
        let code = value.trim().replace("-", "");
        if code.len() != 12 {
            println!("len was: {}", code.len());
            return Err("ISRCs should be 12 characters");
        }
        let country_str = code.get(0..2).unwrap();
        let isrc_issuer = code.get(2..5).unwrap();
        let refyear_and_designation = code.get(5..12).unwrap();
        if rust_iso3166::from_alpha2(country_str).is_none() && !overflow_ccs.contains(&country_str)
        {
            Err("Country code not valid")
        } else if !isrc_issuer.chars().nth(0).unwrap().is_alphanumeric()
            || !isrc_issuer.chars().nth(1).unwrap().is_alphanumeric()
        {
            Err("ISRC issuer registrant code is invalid.")
        } else if !refyear_and_designation
            .chars()
            .enumerate()
            .all(|(_, c)| c.is_numeric())
        {
            Err("Reference year or designation code are invalid.")
        } else {
            Ok(Isrc::Code(
                country_str.to_owned() + isrc_issuer + refyear_and_designation,
            ))
        }
    }
}
