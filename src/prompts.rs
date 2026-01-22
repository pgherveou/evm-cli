use alloy::dyn_abi::DynSolValue;
use alloy::json_abi::Param;

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
            format!("{param_name}: {}", format_return_value(arg))
        })
        .collect();

    format!("{}({})", name, params_str.join(", "))
}

pub fn format_return_value(value: &DynSolValue) -> String {
    match value {
        DynSolValue::Address(a) => format!("{a:?}"),
        DynSolValue::Bool(b) => b.to_string(),
        DynSolValue::Bytes(b) => format!("0x{}", hex::encode(b)),
        DynSolValue::FixedBytes(b, _) => format!("0x{}", hex::encode(b)),
        DynSolValue::Int(i, _) => i.to_string(),
        DynSolValue::Uint(u, _) => u.to_string(),
        DynSolValue::String(s) => format!("\"{s}\""),
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
