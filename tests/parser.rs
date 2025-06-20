
#[cfg(test)]
mod parser_test {
    use test_case::test_case;
    use firrtl3_parser::lexer::*;
    use firrtl3_parser::firrtl::*;

     #[test]
     fn ports() {
         let source =
             r#"
input clock : Clock
input reset : UInt<1>
output io : { flip value1 : UInt<16>, flip value2 : UInt<16>, flip loadingValues : UInt<1>, outputGCD : UInt<16>, outputValid : UInt<1>}
"#;
         let lexer = FIRRTLLexer::new(source);
         let parser = PortsParser::new();
         let ast = parser.parse(lexer).unwrap();
         println!("{:?}", ast);
     }

     #[test]
     fn reg_stmt() {
         let source =
             r#"
reg x : UInt, clock with :
  reset => (UInt<1>("h0"), x) @[GCD.scala 14:15]
"#;
         let lexer = FIRRTLLexer::new(source);
         let parser = StmtParser::new();
         let ast = parser.parse(lexer).unwrap();
         println!("{:?}", ast);
     }

     #[test]
     fn printf_stmt() {
         let source =
             r#"
printf(clock, _GEN_6, "Assertion failed\n    at Arbiter.scala:77 assert((prefixOR zip winner) map { case (p,w) => !p || !w } reduce {_ && _})\n") : printf @[generators/rocket-chip/src/main/scala/tilelink/Arbiter.scala 77:13]
"#;
         let lexer = FIRRTLLexer::new(source);
         let parser = StmtParser::new();
         let ast = parser.parse(lexer).unwrap();
         println!("{:?}", ast);
     }

     #[test]
     fn stop_stmt() {
         let source =
             r#"
stop(clock, _GEN_6, 1) : assert @[generators/rocket-chip/src/main/scala/tilelink/Arbiter.scala 77:13]
"#;
         let lexer = FIRRTLLexer::new(source);
         let parser = StmtParser::new();
         let ast = parser.parse(lexer).unwrap();
         println!("{:?}", ast);
     }

     #[test]
     fn mem_stmt() {
         let source =
             r#"
mem ram_flattened : @[src/main/scala/chisel3/util/Decoupled.scala 256:91]
  data-type => UInt<118>
  depth => 2
  read-latency => 0
  write-latency => 1
  reader => io_deq_bits_MPORT
  writer => MPORT
  read-under-write => undefined

"#;
         let lexer = FIRRTLLexer::new(source);
         let parser = StmtParser::new();
         let ast = parser.parse(lexer).unwrap();
         println!("{:?}", ast);
     }

     #[test_case("Adder" ; "Adder")]
     #[test_case("GCD" ; "GCD")]
     #[test_case("GCDDelta" ; "GCDDelta")]
     #[test_case("Fir" ; "Fir")]
     #[test_case("BitSel1" ; "BitSel1")]
     #[test_case("BitSel2" ; "BitSel2")]
     #[test_case("LCS1" ; "LCS1")]
     #[test_case("LCS2" ; "LCS2")]
     #[test_case("LCS3" ; "LCS3")]
     #[test_case("LCS4" ; "LCS4")]
     #[test_case("LCS5" ; "LCS5")]
     #[test_case("LCS6" ; "LCS6")]
     #[test_case("LCS7" ; "LCS7")]
     #[test_case("LCS8" ; "LCS8")]
     #[test_case("CombHierarchy" ; "CombHierarchy")]
     #[test_case("DecoupledMux" ; "DecoupledMux")]
     #[test_case("DynamicIndexing" ; "DynamicIndexing")]
     #[test_case("MultiWhen" ; "MultiWhen")]
     #[test_case("MyQueue" ; "MyQueue")]
     #[test_case("NestedWhen" ; "NestedWhen")]
     #[test_case("RegFile" ; "RegFile")]
     #[test_case("RegInitWire" ; "RegInitWire")]
     #[test_case("RegVecInit" ; "RegVecInit")]
     #[test_case("Subtracter" ; "Subtracter")]
     #[test_case("Top" ; "Top")]
     #[test_case("SinglePortSRAM" ; "SinglePortSRAM")]
     #[test_case("DualReadSingleWritePortSRAM" ; "DualReadSingleWritePortSRAM")]
     #[test_case("OneReadOneReadWritePortSRAM" ; "OneReadOneReadWritePortSRAM")]
     #[test_case("OneReadOneWritePortSRAM" ; "OneReadOneWritePortSRAM")]
     #[test_case("Cache" ; "Cache")]
     #[test_case("PointerChasing" ; "PointerChasing")]
     #[test_case("FireSimRocket" ; "FireSimRocket")]
     #[test_case("FireSimLargeBoom" ; "FireSimLargeBoom")]
     #[test_case("FireSimQuadRocketFAME5" ; "FireSimQuadRocketFAME5")]
     fn run(name: &str) -> Result<(), std::io::Error> {
         let file = format!("./test-inputs/{}.fir", name);
         let source = std::fs::read_to_string(&file)?;
         let lexer = FIRRTLLexer::new(&source);
         let parser = CircuitParser::new();
         let _ast = parser.parse(lexer).expect("FAILED");
         Ok(())
     }
}
