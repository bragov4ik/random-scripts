use anyhow::{anyhow, Context, Result};
use syn::{parse_file, Expr, Item, Lit, Stmt, ExprMethodCall, ExprCall};

use std::error::Error;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let rust_code = fs::read_to_string("input.rs")?; // Replace with the path to your Rust file

    let syntax_tree = parse_file(&rust_code)?;

    // Open a CSV file for writing
    let mut output_file = fs::File::create("output.csv")?;
    writeln!(
        output_file,
        "function name,preset,time,reads,writes,proof_size"
    )?;

    for item in syntax_tree.items.iter() {
        if let Item::Impl(impl_block) = item {
            for method in &impl_block.items {
                if let syn::ImplItem::Method(method) = method {
                    match find_function_info(&method) {
                        Ok((name, preset, time, reads, writes, proof_size)) => {
                            writeln!(
                                output_file,
                                "{},{},{},{},{},{}",
                                name, preset, time, reads, writes, proof_size
                            )?;
                        }
                        Err(e) => eprintln!("error! {:?}", e),
                    }
                }
            }
        }
    }

    println!("CSV file generated successfully!");
    Ok(())
}

fn find_function_info(
    method: &syn::ImplItemMethod,
) -> Result<(String, String, u128, u128, u128, u128)> {
    let fn_name = method.sig.ident.to_string();
    let (fn_name, fn_preset_n) = fn_name.rsplit_once("_").expect("Unexpected fn name");
    parse_function_body(method)
        .context(format!("parsing fn {}_{}", fn_name, fn_preset_n))
        .map(|(a, b, c, d)| (fn_name.to_owned(), fn_preset_n.to_owned(), a, b, c, d))
}

fn parse_int_lit(a: &Expr) -> Result<u128> {
    let Expr::Lit(expr) = a else { return Err(anyhow!("Did not expect non-literal expr")) };
    let Lit::Int(ref lit) = expr.lit else { return Err(anyhow!("Couldn't parse non-int literal")) };
    Ok(lit.base10_parse()?)
}

fn parse_saturating_add_body(method_call: &ExprMethodCall) -> Result<u128> {
    parse_int_lit(&method_call.args[0]).context("parsing literal in saturaring add (first) arg")
}

fn parse_from_parts_call(call: &ExprCall) -> Result<(u128, u128)> {
    let time = parse_int_lit(&call.args[0])
        .context("parsing first argument to the 3rd call from the end (`from_parts`?)")?;
    let proof_size = parse_int_lit(&call.args[1])
        .context("parsing second argument to the 3rd call from the end ")?;
    Ok((time, proof_size))
}

struct WeightCall {
    from_parts_call: ExprCall,
    reads_saturating_add: Option<ExprMethodCall>,
    writes_saturating_add: Option<ExprMethodCall>,
}

impl WeightCall {
    fn from_parts_only(call: ExprCall) -> Self {
        WeightCall { from_parts_call: call, reads_saturating_add: None, writes_saturating_add: None}
    }
}

#[derive(Debug, PartialEq, Eq)]
enum AddType {
    Reads,
    Writes
}

fn resolve_add(add: &ExprMethodCall) -> Result<AddType> {
    let body = &add.args[0];
    let Expr::MethodCall(method_call) = body else { return Err(anyhow!("Expected method call")) };
    match &*format!("{}", method_call.method) {
        "reads" => Ok(AddType::Reads),
        "writes" => Ok(AddType::Writes),
        other => Err(anyhow!("Expected `reads` or `writes`, got `{}`", other))
    }
}

fn parse_expr_parts(weight_expr: &Expr) -> Result<WeightCall> {
    match weight_expr {
        Expr::MethodCall(saturating_add_last) => {
            match saturating_add_last.receiver.as_ref() {
                Expr::MethodCall(saturating_add_first) => {
                    // two adds
                    let Expr::Call(from_parts_call) = saturating_add_first.receiver.as_ref().clone() else { return Err(anyhow!("Couldn't parse `from_parts` call")) };
                    match (resolve_add(saturating_add_first)?, resolve_add(saturating_add_last)?) {
                        (AddType::Reads, AddType::Writes) => Ok(WeightCall{ from_parts_call, reads_saturating_add: Some(saturating_add_first.clone()), writes_saturating_add: Some(saturating_add_last.clone()) }),
                        (AddType::Writes, AddType::Reads) => Ok(WeightCall{ from_parts_call, reads_saturating_add: Some(saturating_add_last.clone()), writes_saturating_add: Some(saturating_add_first.clone()) }),
                        c => Err(anyhow!("Expected one reads and one writes, got {:?}", c))
                    }
                }
                Expr::Call(from_parts_call) => {
                    // only one add
                    let mut call = WeightCall::from_parts_only(from_parts_call.clone());
                    match resolve_add(saturating_add_last)? {
                        AddType::Reads => call.reads_saturating_add = Some(saturating_add_last.clone()),
                        AddType::Writes => call.writes_saturating_add = Some(saturating_add_last.clone()),
                    };
                    Ok(call)
                }
                other => {
                    Err(anyhow!("Expected chained calls, got {:?}", other))
                }
            }
        },
        Expr::Call(from_parts_call) => {
            // no adds
            Ok(WeightCall::from_parts_only(from_parts_call.clone()))
        }
        other => { Err(anyhow!("Expected weight expr, got {:?}", other)) }
    }
}

fn parse_function_body(method: &syn::ImplItemMethod) -> Result<(u128, u128, u128, u128)> {
    let Stmt::Expr(weight_expr) = method
        .block
        .stmts
        .last()
        .ok_or(anyhow!("Expected fn to be non-empty "))? else {
        return Err(anyhow!("Expected an expr at the end of the fn"))
    };

    let WeightCall {from_parts_call, writes_saturating_add, reads_saturating_add} = parse_expr_parts(weight_expr)?;

    let (time, proof_size) = parse_from_parts_call(&from_parts_call)?;

    let reads = reads_saturating_add.map(|body| parse_saturating_add_body(&body)).transpose()?.unwrap_or(0);
    let writes = writes_saturating_add.map(|body| parse_saturating_add_body(&body)).transpose()?.unwrap_or(0);
    Ok((time, proof_size, reads, writes))
}
