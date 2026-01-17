use alloy::json_abi::{Function, JsonAbi, StateMutability};

#[derive(Debug, Clone)]
pub enum MethodSelection {
    Constructor,
    Function(Function),
}

pub struct MethodDisplay {
    pub label: String,
    pub tag: &'static str,
    pub selection: MethodSelection,
}

impl std::fmt::Display for MethodDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<50} [{}]", self.label, self.tag)
    }
}

pub fn list_methods(abi: &JsonAbi, include_constructor: bool) -> Vec<MethodDisplay> {
    let mut methods = Vec::new();

    // Add constructor option if requested
    if include_constructor {
        let label = if let Some(ctor) = &abi.constructor {
            let params = format_params(&ctor.inputs);
            format!("constructor({})", params)
        } else {
            "constructor()".to_string()
        };

        methods.push(MethodDisplay {
            label,
            tag: "deploy",
            selection: MethodSelection::Constructor,
        });
    }

    // Collect all functions
    let mut functions: Vec<_> = abi.functions().collect();

    // Sort: view/pure first, then by name
    functions.sort_by(|a, b| {
        let a_is_view = matches!(
            a.state_mutability,
            StateMutability::View | StateMutability::Pure
        );
        let b_is_view = matches!(
            b.state_mutability,
            StateMutability::View | StateMutability::Pure
        );

        match (a_is_view, b_is_view) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    for func in functions {
        let params = format_params(&func.inputs);
        let returns = format_outputs(&func.outputs);

        let label = if returns.is_empty() {
            format!("{}({})", func.name, params)
        } else {
            format!("{}({}) -> {}", func.name, params, returns)
        };

        let tag = match func.state_mutability {
            StateMutability::View | StateMutability::Pure => "view",
            StateMutability::Payable => "payable",
            StateMutability::NonPayable => "send",
        };

        methods.push(MethodDisplay {
            label,
            tag,
            selection: MethodSelection::Function(func.clone()),
        });
    }

    methods
}

fn format_params(params: &[alloy::json_abi::Param]) -> String {
    params
        .iter()
        .map(|p| {
            if p.name.is_empty() {
                p.ty.to_string()
            } else {
                format!("{}: {}", p.name, p.ty)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_outputs(outputs: &[alloy::json_abi::Param]) -> String {
    if outputs.is_empty() {
        return String::new();
    }

    if outputs.len() == 1 {
        return outputs[0].ty.to_string();
    }

    let types: Vec<_> = outputs.iter().map(|p| p.ty.to_string()).collect();
    format!("({})", types.join(", "))
}
