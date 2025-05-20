
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
     fn run(name: &str) -> Result<(), std::io::Error> {
         let file = format!("./test-inputs/{}.fir", name);
         let source = std::fs::read_to_string(&file)?;
         let lexer = FIRRTLLexer::new(&source);
         let parser = CircuitParser::new();
         let _ast = parser.parse(lexer).expect("FAILED");
         Ok(())
     }

//     #[test]
//     fn rocketconfig() -> Result<(), std::io::Error> {
//         let source = std::fs::read_to_string("./test-inputs/chipyard.harness.TestHarness.RocketConfig.fir")?;
//         let lexer = FIRRTLLexer::new(&source);
//         let parser = CircuitParser::new();
//         let ast = parser.parse(lexer).expect("FAILED");
//         Ok(())
//     }
// 
//     #[test]
//     fn boomconfig() -> Result<(), std::io::Error> {
//         let source = std::fs::read_to_string("./test-inputs/chipyard.harness.TestHarness.LargeBoomV3Config.fir")?;
//         let lexer = FIRRTLLexer::new(&source);
//         let parser = CircuitParser::new();
//         let ast = parser.parse(lexer).expect("FAILED");
//         Ok(())
//     }
// 
//     #[test]
//     fn double_indexing() -> Result<(), std::io::Error> {
//         let source = r#"connect io_debug_fetch_pc_0_REG, pcs[io.debug_ftq_idx[0]] @[generators/boom/src/main/scala/v3/ifu/fetch-target-queue.scala 363:36]"#;
//         let lexer = FIRRTLLexer::new(&source);
//         let parser = StmtParser::new();
//         let ast = parser.parse(lexer).expect("FAILED");
//         println!("{:?}", ast);
//         Ok(())
//     }
// 
//     #[test]
//     fn rocket_modules() -> Result<(), std::io::Error> {
//         for entry in std::fs::read_dir("./test-inputs/rocket-modules/")? {
//             let entry = entry?;
//             let path = entry.path();
// 
//             // Check if it's a file (not a directory)
//             if path.is_file() {
//                 match std::fs::read_to_string(&path) {
//                     Ok(source) => {
//                         let lexer = FIRRTLLexer::new(&source);
//                         let parser = CircuitModuleParser::new();
// 
//                         println!("Parsing file: {:?}", path);
//                         let ast = parser.parse(lexer).expect("TOWORK");
//                     }
//                     Err(e) => {
//                         eprintln!("Could not read file {}: {}", path.display(), e);
//                     }
//                 }
//             }
//         }
//         Ok(())
//     }
}
