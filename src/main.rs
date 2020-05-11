#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator1::ExprParser::new().parse("22").is_ok());
    assert!(calculator1::ExprParser::new().parse("(22)").is_ok());
    assert!(calculator1::ExprParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::ExprParser::new().parse("((22)").is_err());
    assert!(calculator1::ExprParser::new().parse("1+").is_err());
    assert!(calculator1::ExprParser::new().parse("1+*").is_err());
    assert!(calculator1::ExprParser::new().parse("*2").is_err());
    assert!(calculator1::ExprParser::new().parse("1+1").is_ok());
    assert!(calculator1::ExprParser::new().parse("2*2").is_ok());
    assert!(calculator1::ExprParser::new().parse("2*(2)").is_ok());
    assert!(calculator1::ExprParser::new().parse("2*(2+1)").is_ok());
}
