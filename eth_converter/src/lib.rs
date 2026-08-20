use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EthUnit { Wei, GWei, Eth }

impl FromStr for EthUnit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "wei" => Ok(EthUnit::Wei),
            "gwei" => Ok(EthUnit::GWei),
            "eth" => Ok(EthUnit::Eth),
            _ => Err(format!("{s} is not a valid unit")),
        }
    }
}

impl EthUnit {
    const fn decimals(self) -> u32 {
        match self {
            Self::Wei => 0,
            Self::GWei => 9,
            Self::Eth => 18,
        }
    }

    pub fn convert(value: u128, from: Self, to: Self) -> Result<u128, String> {
        let (f, t) = (from.decimals(), to.decimals());
        if f >= t {
            value.checked_mul(10u128.pow(f - t)).ok_or(format!("{value} {from:?} overflows u128 as {to:?}"))
        } else {
            let factor = 10u128.pow(t - f);
            if value % factor == 0 {
                Ok(value / factor)
            } else {
                Err(format!("{value} {from:?} is not a whole number of {to:?}"))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_cases() {
        assert_eq!(EthUnit::from_str("eth").unwrap(), EthUnit::Eth);
        assert_eq!(EthUnit::from_str("Eth").unwrap(), EthUnit::Eth);
        assert_eq!(EthUnit::from_str("ETH").unwrap(), EthUnit::Eth);
    }

    #[test]
    fn convert() {
        assert_eq!(EthUnit::convert(1u128, EthUnit::Eth, EthUnit::Wei).unwrap(), 10u128.pow(18));
    }
}
