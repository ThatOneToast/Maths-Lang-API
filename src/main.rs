
mod helpers;


fn main() {
    let maths = helpers::Maths::new(
        Option::from("/Users/toast/Documents/git/Maths-Lang/target/debug/Maths".to_string())
    );
    
    
    let expression = r#"
let num = 50
;num"#;
    
    maths.run(expression)
    
    
}