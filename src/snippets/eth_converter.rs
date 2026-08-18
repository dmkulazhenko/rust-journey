use std::str::FromStr;

// const GIGA_FACTOR: u128 = 1_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EthUnit { Wei, GWei, Eth }

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

    fn convert(value: u128, from: Self, to: Self) -> Result<u128, String> {
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

    // fn get_cast_func(self, other: EthUnit) -> fn(u128) -> u128 {
    //     match (self, other) {
    //         (EthUnit::Wei, EthUnit::GWei) | (EthUnit::GWei, EthUnit::Eth) => |n| n / GIGA_FACTOR,
    //         (EthUnit::Wei, EthUnit::Eth) => |n| n / (GIGA_FACTOR * GIGA_FACTOR),
    //         (EthUnit::Wei, EthUnit::Wei) | (EthUnit::GWei, EthUnit::GWei) | (EthUnit::Eth, EthUnit::Eth) => |n| n,
    //         (EthUnit::GWei, EthUnit::Wei) | (EthUnit::Eth, EthUnit::GWei) => |n| n.saturating_mul(GIGA_FACTOR),
    //         (EthUnit::Eth, EthUnit::Wei) => |n| n.saturating_mul(GIGA_FACTOR * GIGA_FACTOR),
    //     }
    // }
}


fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let source_unit: EthUnit = args.next()
        .ok_or("missing source unit")?.parse()?;
    let target_unit: EthUnit = args.next()
        .ok_or("missing target unit")?.parse()?;
    let value: u128 = args.next()
        .ok_or("missing value")?.parse().map_err(|e| { format!("value is not valid u128: {e}") })?;

    println!("{}", EthUnit::convert(value, source_unit, target_unit)?);
    
    Ok(())
}
