use conversion_error_handling_workshop::{
    and_then_chain, first_present, map_err_convert, multi_step_pipeline, ok_or_convert,
    read_and_parse, unwrap_or_default_when_none,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("None -> 0: {}", unwrap_or_default_when_none(None));
    println!("Some(42) -> 42: {}", unwrap_or_default_when_none(Some(42)));

    println!("\nmap_err_convert('123') = {:?}", map_err_convert("123"));
    println!("map_err_convert('oops') = {:?}", map_err_convert("oops"));

    println!("\nand_then_chain('42') = {:?}", and_then_chain("42"));
    println!("and_then_chain('oops') = {:?}", and_then_chain("oops"));

    println!("\nread_and_parse('100') = {:?}", read_and_parse("100"));
    println!("read_and_parse('oops') = {:?}", read_and_parse("oops"));

    println!("\nmulti_step_pipeline('5') = {:?}", multi_step_pipeline("5"));
    println!("multi_step_pipeline('oops') = {:?}", multi_step_pipeline("oops"));

    println!(
        "\nfirst_present([None, Some('b')]) = {:?}",
        first_present(&[None, Some("b"), Some("c")])
    );

    println!("\nok_or_convert(None) = {:?}", ok_or_convert(None));

    Ok(())
}
