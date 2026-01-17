use alloy::dyn_abi::{DynSolType, DynSolValue};
use alloy::json_abi::Param;
use alloy::primitives::{Address, I256, U256};
use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("cancelled")]
pub struct Cancelled;

pub fn is_cancelled(err: &anyhow::Error) -> bool {
    err.downcast_ref::<Cancelled>().is_some()
}

fn check_cancelled<T>(result: Result<T, inquire::InquireError>) -> Result<T> {
    match result {
        Ok(v) => Ok(v),
        Err(inquire::InquireError::OperationCanceled) => Err(Cancelled.into()),
        Err(inquire::InquireError::OperationInterrupted) => Err(Cancelled.into()),
        Err(e) => Err(e.into()),
    }
}

pub fn prompt_for_params(params: &[Param]) -> Result<Vec<DynSolValue>> {
    let mut values = Vec::new();

    for param in params {
        let type_str = param.ty.as_str();
        let name = if param.name.is_empty() {
            type_str.to_string()
        } else {
            format!("{} ({})", param.name, type_str)
        };

        let sol_type = type_str
            .parse::<DynSolType>()
            .with_context(|| format!("Unsupported type: {}", type_str))?;

        let value = prompt_for_type(&name, &sol_type)?;
        values.push(value);
    }

    Ok(values)
}

fn prompt_for_type(name: &str, sol_type: &DynSolType) -> Result<DynSolValue> {
    match sol_type {
        DynSolType::Address => prompt_address(name),
        DynSolType::Bool => prompt_bool(name),
        DynSolType::Bytes => prompt_bytes(name),
        DynSolType::FixedBytes(size) => prompt_fixed_bytes(name, *size),
        DynSolType::Int(bits) => prompt_int(name, *bits),
        DynSolType::Uint(bits) => prompt_uint(name, *bits),
        DynSolType::String => prompt_string(name),
        DynSolType::Array(inner) => prompt_array(name, inner),
        DynSolType::FixedArray(inner, size) => prompt_fixed_array(name, inner, *size),
        DynSolType::Tuple(types) => prompt_tuple(name, types),
        DynSolType::Function => {
            anyhow::bail!("Function type not supported for input")
        }
    }
}

fn prompt_address(name: &str) -> Result<DynSolValue> {
    let input = check_cancelled(
        inquire::Text::new(&format!("{}:", name))
            .with_placeholder("0x...")
            .with_validator(|s: &str| {
                if s.parse::<Address>().is_ok() {
                    Ok(inquire::validator::Validation::Valid)
                } else {
                    Ok(inquire::validator::Validation::Invalid(
                        "Invalid address format (expected 0x + 40 hex chars)".into(),
                    ))
                }
            })
            .prompt(),
    )?;

    let address: Address = input.parse().context("Invalid address")?;
    Ok(DynSolValue::Address(address))
}

fn prompt_bool(name: &str) -> Result<DynSolValue> {
    let options = vec!["true", "false"];
    let selection = check_cancelled(
        inquire::Select::new(&format!("{}:", name), options).prompt(),
    )?;

    Ok(DynSolValue::Bool(selection == "true"))
}

fn prompt_bytes(name: &str) -> Result<DynSolValue> {
    let input = check_cancelled(
        inquire::Text::new(&format!("{} (hex):", name))
            .with_placeholder("0x...")
            .prompt(),
    )?;

    let input = input.strip_prefix("0x").unwrap_or(&input);
    let bytes = hex::decode(input).context("Invalid hex string")?;
    Ok(DynSolValue::Bytes(bytes))
}

fn prompt_fixed_bytes(name: &str, size: usize) -> Result<DynSolValue> {
    let input = check_cancelled(
        inquire::Text::new(&format!("{} (bytes{}):", name, size))
            .with_placeholder("0x...")
            .with_validator(move |s: &str| {
                let s = s.strip_prefix("0x").unwrap_or(s);
                if hex::decode(s).map(|b| b.len() == size).unwrap_or(false) {
                    Ok(inquire::validator::Validation::Valid)
                } else {
                    Ok(inquire::validator::Validation::Invalid(
                        format!("Expected {} bytes in hex", size).into(),
                    ))
                }
            })
            .prompt(),
    )?;

    let input = input.strip_prefix("0x").unwrap_or(&input);
    let bytes = hex::decode(input).context("Invalid hex string")?;

    // Pad or truncate to exact size
    let mut fixed = vec![0u8; size];
    let copy_len = bytes.len().min(size);
    fixed[..copy_len].copy_from_slice(&bytes[..copy_len]);

    Ok(DynSolValue::FixedBytes(
        alloy::primitives::FixedBytes::from_slice(&fixed),
        size,
    ))
}

fn prompt_int(name: &str, bits: usize) -> Result<DynSolValue> {
    let input = check_cancelled(
        inquire::Text::new(&format!("{} (int{}):", name, bits))
            .with_placeholder("0")
            .prompt(),
    )?;

    let value: I256 = input.parse().context("Invalid number")?;

    Ok(DynSolValue::Int(value, bits))
}

fn prompt_uint(name: &str, bits: usize) -> Result<DynSolValue> {
    let input = check_cancelled(
        inquire::Text::new(&format!("{} (uint{}):", name, bits))
            .with_placeholder("0")
            .prompt(),
    )?;

    let value: U256 = if input.starts_with("0x") {
        U256::from_str_radix(input.trim_start_matches("0x"), 16)
            .context("Invalid hex number")?
    } else {
        input.parse().context("Invalid decimal number")?
    };

    Ok(DynSolValue::Uint(value, bits))
}

fn prompt_string(name: &str) -> Result<DynSolValue> {
    let input = check_cancelled(
        inquire::Text::new(&format!("{}:", name)).prompt(),
    )?;

    Ok(DynSolValue::String(input))
}

fn prompt_array(name: &str, inner_type: &DynSolType) -> Result<DynSolValue> {
    let mut values = Vec::new();
    let mut index = 0;

    loop {
        let element_name = format!("{}[{}]", name, index);
        let value = prompt_for_type(&element_name, inner_type)?;
        values.push(value);
        index += 1;

        let add_more = check_cancelled(
            inquire::Confirm::new("Add another element?")
                .with_default(false)
                .prompt(),
        )?;

        if !add_more {
            break;
        }
    }

    Ok(DynSolValue::Array(values))
}

fn prompt_fixed_array(name: &str, inner_type: &DynSolType, size: usize) -> Result<DynSolValue> {
    let mut values = Vec::with_capacity(size);

    for i in 0..size {
        let element_name = format!("{}[{}]", name, i);
        let value = prompt_for_type(&element_name, inner_type)?;
        values.push(value);
    }

    Ok(DynSolValue::FixedArray(values))
}

fn prompt_tuple(name: &str, types: &[DynSolType]) -> Result<DynSolValue> {
    println!("Enter tuple fields for {}:", name);

    let mut values = Vec::with_capacity(types.len());

    for (i, inner_type) in types.iter().enumerate() {
        let field_name = format!("{}.{}", name, i);
        let value = prompt_for_type(&field_name, inner_type)?;
        values.push(value);
    }

    Ok(DynSolValue::Tuple(values))
}

pub fn format_method_call(name: &str, params: &[Param], args: &[DynSolValue]) -> String {
    let params_str: Vec<String> = params
        .iter()
        .zip(args.iter())
        .map(|(param, arg)| {
            let param_name = if param.name.is_empty() {
                param.ty.clone()
            } else {
                param.name.clone()
            };
            format!("{}: {}", param_name, format_return_value(arg))
        })
        .collect();

    format!("{}({})", name, params_str.join(", "))
}

pub fn format_return_value(value: &DynSolValue) -> String {
    match value {
        DynSolValue::Address(a) => format!("{:?}", a),
        DynSolValue::Bool(b) => b.to_string(),
        DynSolValue::Bytes(b) => format!("0x{}", hex::encode(b)),
        DynSolValue::FixedBytes(b, _) => format!("0x{}", hex::encode(b)),
        DynSolValue::Int(i, _) => i.to_string(),
        DynSolValue::Uint(u, _) => u.to_string(),
        DynSolValue::String(s) => format!("\"{}\"", s),
        DynSolValue::Array(arr) | DynSolValue::FixedArray(arr) => {
            let items: Vec<_> = arr.iter().map(format_return_value).collect();
            format!("[{}]", items.join(", "))
        }
        DynSolValue::Tuple(fields) => {
            let items: Vec<_> = fields.iter().map(format_return_value).collect();
            format!("({})", items.join(", "))
        }
        DynSolValue::Function(f) => format!("0x{}", hex::encode(f)),
    }
}
