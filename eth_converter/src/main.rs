use eth_converter::EthUnit;


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
