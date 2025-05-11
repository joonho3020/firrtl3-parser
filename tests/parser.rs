
#[cfg(test)]
mod parser_test {
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
     fn gcd() -> Result<(), std::io::Error> {
         let source = std::fs::read_to_string("./test-inputs/GCD.fir")?;
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
