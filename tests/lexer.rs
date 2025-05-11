
#[cfg(test)]
mod lexer_test {
    use firrtl3_parser::lexer::*;
    use test_case::test_case;

    fn run_lexer(source: &str) {
        let mut lex = FIRRTLLexer::new(source);
        while let Some(ts) = lex.next_token() {
            println!("{:?}", ts);
            match ts.token {
                Token::Error => {
                    println!("{:?}", ts);
                    panic!("Got a error token");
                }
                _ => { }
            }
        }
    }

    #[test_case("Adder" ; "Adder")]
    #[test_case("GCD" ; "GCD")]
    #[test_case("NestedWhen" ; "NestedWhen")]
    #[test_case("LCS1" ; "LCS1")]
    #[test_case("LCS2" ; "LCS2")]
    #[test_case("LCS3" ; "LCS3")]
    #[test_case("LCS4" ; "LCS4")]
    #[test_case("LCS5" ; "LCS5")]
    #[test_case("LCS6" ; "LCS6")]
    #[test_case("LCS7" ; "LCS7")]
    #[test_case("LCS8" ; "LCS8")]
    #[test_case("BitSel1" ; "BitSel1")]
    #[test_case("BitSel2" ; "BitSel2")]
    #[test_case("RegInitWire" ; "RegInitWire")]
    #[test_case("SinglePortSRAM" ; "SinglePortSRAM")]
    #[test_case("OneReadOneWritePortSRAM" ; "OneReadOneWritePortSRAM")]
    #[test_case("OneReadOneReadWritePortSRAM" ; "OneReadOneReadWritePortSRAM")]
    #[test_case("WireRegInsideWhen" ; "WireRegInsideWhen")]
    #[test_case("MultiWhen" ; "MultiWhen")]
    #[test_case("Cache" ; "Cache")]
    #[test_case("FireSim" ; "FireSim")]
    fn run(input: &str) -> Result<(), std::io::Error> {
        let source = std::fs::read_to_string(&format!("./test-inputs/{}.fir", input))?;
        run_lexer(&source);
        Ok(())
    }
}
