use syn::{parse_file, Item, Stmt, Expr, Lit};
use quote::quote;

use std::fs;
use std::error::Error;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let rust_code = fs::read_to_string("input.rs")?; // Replace with the path to your Rust file

    let syntax_tree = parse_file(&rust_code)?;

    // Open a CSV file for writing
    let mut output_file = fs::File::create("output.csv")?;
    writeln!(output_file, "function name,time,reads,writes,proof_size")?;

    for item in syntax_tree.items.iter() {
        if let Item::Impl(impl_block) = item {
            for method in &impl_block.items {
                if let syn::ImplItem::Method(method) = method {
                    if let Some((name, time, reads, writes, proof_size)) = find_function_info(&method) {
                        writeln!(output_file, "{},{},{},{},{}", name, time, reads, writes, proof_size)?;
                    }
                    panic!()
                }
            }
        }
    }

    println!("CSV file generated successfully!");
    Ok(())
}

fn find_function_info(method: &syn::ImplItemMethod) -> Option<(String, u64, u64, u64, u64)> {
    let fn_name = method.sig.ident.to_string();
    let (fn_name, fn_preset_n) = fn_name.rsplit_once("_").expect("Unexpected fn name");
    let weight_stmt = method.block.stmts.last().expect(&format!("Expected fn {} to be non-empty ", fn_name));
    let Stmt::Expr(Expr::MethodCall(weight_method_call)) = weight_stmt else { panic!("Expected a method call at the end of fn {}", fn_name) };

    let Expr::MethodCall(aboba) = weight_method_call.receiver.as_ref() else { panic!() };
    let Expr::Call(kek) = aboba.receiver.as_ref() else { panic!() };

    fn parse_int_lit(a: Expr) -> u128 {
        let Expr::Lit(expr) = a else { panic!() };
        let Lit::Int(lit) = expr.lit else { panic!() };
        lit.base10_parse().unwrap()
    }
    let time = parse_int_lit(kek.args[0].clone());
    let proof_size = parse_int_lit(kek.args[1].clone());

    dbg!(&aboba.args);
    dbg!(&weight_method_call.args);
    return None;
    // // Parse the function body to extract the values
    // let mut time = 0;
    // let mut proof_size = 0;
    // let mut reads = 0;
    // let mut writes = 0;

    // for line in fn_body.lines() {
    //     if line.contains("Minimum execution time:") {
    //         if let Some(picos) = line.split("Minimum execution time:").last() {
    //             if let Ok(parsed_time) = picos.trim().replace("picoseconds.", "").parse() {
    //                 time = parsed_time;
    //             }
    //         }
    //     }
    //     if line.contains("Estimated:") {
    //         if let Some(estimated) = line.split("Estimated:").last() {
    //             if let Ok(parsed_proof_size) = estimated.trim().replace("`", "").parse() {
    //                 proof_size = parsed_proof_size;
    //             }
    //         }
    //     }
    //     if line.contains(".reads(") {
    //         if let Some(reads_str) = line.split(".reads(").last() {
    //             if let Ok(parsed_reads) = reads_str.replace("_u64))", "").parse() {
    //                 reads = parsed_reads;
    //             }
    //         }
    //     }
    //     if line.contains(".writes(") {
    //         if let Some(writes_str) = line.split(".writes(").last() {
    //             if let Ok(parsed_writes) = writes_str.replace("_u64))", "").parse() {
    //                 writes = parsed_writes;
    //             }
    //         }
    //     }
    // }

    // if time > 0 {
    //     Some((fn_name, time, reads, writes, proof_size))
    // } else {
    //     None
    // }
}
