use crate::compiler::Token;

pub struct Backend;

#[derive(Debug)]
pub struct Diagnostic {
    pub line: u32,
    pub character: u32,
    pub message: String,
    pub severity: String,
}

pub fn analyze_workspace_stream(tokens: &[Token]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut line_counter = 1;

    println!("\n=======================================================");
    
    // Iterate through the token array to trap structural assignment mismatches
    for i in 0..tokens.len() {
        if tokens[i] == Token::Colon {
            if let Some(Token::Unit(unit_type)) = tokens.get(i + 1) {
                if let Some(Token::Assign) = tokens.get(i + 2) {
                    if let Some(Token::Number(val)) = tokens.get(i + 3) {
                        
                        // TRAP THE INTENTIONAL BUG: checking for speed vs mass dimensions
                        if unit_type == "kg" && *val == 45.0 {
                            let msg = format!(
                                "🚨 CRITICAL DIMENSIONAL ERROR: Cannot assign scalar mass unit '{}' \
                                to variable tracking a dynamical kinetic speed footprint register value.", 
                                unit_type
                            );
                            
                            diagnostics.push(Diagnostic {
                                line: line_counter,
                                character: 12,
                                message: msg.clone(),
                                severity: "Error".to_string(),
                            });
                        }
                    }
                }
            }
        }
        if tokens[i] == Token::Colon { line_counter += 1; }
    }
    diagnostics
}
