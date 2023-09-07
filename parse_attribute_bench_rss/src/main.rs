use anyhow::{anyhow, Context, Result};
use syn::{parse_file, Expr, Item, Lit, Stmt, ExprMethodCall};

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
    parse_funciton_body(method)
        .context(format!("parsing fn {}_{}", fn_name, fn_preset_n))
        .map(|(a, b, c, d)| (fn_name.to_owned(), fn_preset_n.to_owned(), a, b, c, d))
}

fn parse_funciton_body(method: &syn::ImplItemMethod) -> Result<(u128, u128, u128, u128)> {
    let weight_stmt = method
        .block
        .stmts
        .last()
        .ok_or(anyhow!("Expected fn to be non-empty "))?;
    let Stmt::Expr(Expr::MethodCall(saturating_add_2)) = weight_stmt else { return Err(anyhow!("Expected a method call at the end of the fn")) };

    let Expr::MethodCall(saturating_add_1) = saturating_add_2.receiver.as_ref() else { return Err(anyhow!("Couldn't parse first add")) };
    let Expr::Call(from_parts_call) = saturating_add_1.receiver.as_ref() else { return Err(anyhow!("Couldn't parse `from_parts` call")) };

    fn parse_int_lit(a: &Expr) -> Result<u128> {
        let Expr::Lit(expr) = a else { return Err(anyhow!("Did not expect non-literal expr")) };
        let Lit::Int(ref lit) = expr.lit else { return Err(anyhow!("Couldn't parse non-int literal")) };
        Ok(lit.base10_parse()?)
    }
    let time = parse_int_lit(&from_parts_call.args[0])
        .context("parsing first argument to the 3rd call from the end (`from_parts`?)")?;
    let proof_size = parse_int_lit(&from_parts_call.args[1])
        .context("parsing second argument to the 3rd call from the end ")?;

    fn parse_saturating_add_body(method_call: &ExprMethodCall) -> Result<u128> {
        parse_int_lit(&method_call.args[0]).context("parsing literal in saturaring add (first) arg")
    }

    fn parse_with_name_in_saturating_add_body(body: &Expr, expected_name: &str) -> Result<Option<u128>> {
        let Expr::MethodCall(method_call) = body else { return Err(anyhow!("Expected method call")) };
        if format!("{}", method_call.method) == expected_name {
            parse_saturating_add_body(method_call).map(Some)
        }
        else {
            Ok(None)
        }
    }

    let Some(reads) = parse_with_name_in_saturating_add_body(&saturating_add_1.args[0], "reads")
        .context("parsing first argument to the 2nd saturating_add(?) from the end")? else { return Err(anyhow!("Expected `reads` as the second call from the end")) };
    let Some(writes) = parse_with_name_in_saturating_add_body(&saturating_add_2.args[0], "writes")
        .context("parsing first argument to the 1st saturating_add(?) from the end")? else { return Err(anyhow!("Expected `writes` as the latest call")) };

    Ok((time, proof_size, reads, writes))
}
