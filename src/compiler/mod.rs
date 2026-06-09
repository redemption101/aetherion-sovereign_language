pub mod lsp;
pub mod wasm;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Actor, Stream, Capability, Requires, State, Receive, Emit, Let, Assert,
    // --- V4 UNIVERSAL PHYSICS TOKENS ---
    Fluid, Re, NavierStokes, EField, BField, Maxwell, Gamma, Lorentz, Molecule,
    Mol, React, Drug, Patient, Dose, Toxicity, FMa, NewtonianGuard,
    Identifier(String), Number(f64), Unit(String), Assign, Colon, OpenBrace, CloseBrace
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operator { GreaterThan, LessThan, Equal }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Value {
    pub num: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpCode {
    CheckLease { capability: String },
    AssertVar { name: String, limit: Value },
    SnapshotState { hlc: u64, event: String },
    TerminateProcess,
    // --- V4 PHYSICS KERNEL OPCODES ---
    AxbNavierStokesStep { reynolds_number: f64 },
    AxbMaxwellSolve { divergence: f64 },
    AxbLorentzFactor { velocity_c: f64 },
    AxbReact { mass_balance: bool },
    AxbPharmacokinetic { dose_mg: f64, patient_kg: f64 },
    AxbSandboxFence { memory_safe: bool },
    AxbNewtonianGuard { force: f64, mass: f64, accel: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ASTNode {
    ActorDecl { 
        name: String, 
        state_variables: Vec<VariableDecl>, 
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDecl { pub name: String, pub unit_type: String, pub literal_value: f64 }

// Stubbed lexer/parser for VM routing
pub struct Lexer<'a> { chars: std::iter::Peekable<std::str::Chars<'a>> }
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self { Self { chars: input.chars().peekable() } }
    pub fn next_token(&mut self) -> Option<Token> { None }
}
