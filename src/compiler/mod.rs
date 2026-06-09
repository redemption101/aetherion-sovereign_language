pub mod lsp;
pub mod wasm;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Actor, Stream, Capability, Requires, State, Receive, Emit, Let, Assert, GreaterThan, LessThan,
    Agent, Model, Input, Output, If, Else, Supervisor, Restart, Strategy, ExponentialBackoff, Int64,
    And, Or, Not, Partition, Retention, Crdt, Merge, Lww, At, Edge, Cloud, Lease, Expiry, Quota, Invariant,
    Qubit, Entangle, GeoCoordinate, LatencyMs, TargetEngine, AutoRepair, HealthThreshold, PlanQuery,
    Planet, CelestialMesh, NeuralWeight, RoutingBias, ZkProve, VerifyTrust, HiveConsensus, DevicePool,
    RaftLeader, RaftFollower, TermIndex, HeartbeatMs, QpuRegister, PhaseShift, Hadamard, MeasureQpu,
    LunarRelay, DistributedThread, ThreadPoolShared, ZeroKnowledgeTask, SpaceLightDelay,
    Publish, Marketplace, DepositWallet, TransactionFee, EscrowLease, CreditUnit,
    Identifier(String), Number(f64), Unit(String), Assign, Colon, OpenBrace, CloseBrace
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operator { GreaterThan, LessThan }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogicalOp { And, Or }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Value {
    pub num: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpCode {
    AssertVar { name: String, limit: Value },
    CheckLease { capability: String },
    LoadVar { name: String, expected_unit: String },
    LoadConst(Value),
    Compare(Operator),
    Logical(LogicalOp),
    EmitStream(String),
    SnapshotState { hlc: u64, event: String },
    TerminateProcess,
    AxbMeshRoutingBias { neuron_id: String, signal_weight: f64 },
    AxbZkProofVerify { contract_hash: String },
    AxbHiveConsensusSync { global_state_root: String, device_count: u64 },
    AxbPlanetaryRelativisticDelay { origin_planet: String, light_delay_64: f64 },
    AxbRaftLeaderElection { term: u64, quorum_count: u32 },
    AxbQpuHardwareHadamard { qubit_id: String },
    AxbQpuPhaseShift { qubit_id: String, theta: f64 },
    AxbQpuMeasureCollapse { qubit_id: String, target_register: String },
    AxbLspRenderDiagnostic { line: u32, msg: String },
    AxbWasmVerifySignedManifest { actor_hash: String, lease_sec: u32 },
    AxbNetworkTcpBroadcast { port: u16, payload_bytes: String },
    AxbHivePoolIdleThreads { available_workers: u32, shared_mem_bytes: u64 },
    AxbSpaceTimeMeshRoute { destination_node: String, latency_offset_ms: u64 },
    AxbEcosystemPublishActor { actor_name: String, license_fee_credits: f64 },
    AxbEcosystemProcessLeasePayment { consumer_wallet: String, publisher_split: f64, protocol_fee: f64 },
    AxbIngestKnowledgeNode { category: String, node_identity: String },
    AxbLinkAncestralLineage { parent_scope: String, child_target: String },
    // --- FINAL Horizon HYBRID OPCODES ---
    AxbZkVerifyLeaseQuota { proof_hash: String, allocated_quota: u64 },
    AxbK8sMultiRegionReconcile { target_region: String, dynamic_replica_count: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ASTNode {
    CapabilityDecl { name: String },
    StreamDecl { name: String, partition_key: String, retention_days: u32 },
    CrdtDecl { name: String, strategy: String },
    InvariantDecl { target_actor: String, expression: String },
    ActorDecl { 
        name: String, 
        placement: String,
        requirements: Vec<CapabilityLeaseDecl>,
        state_variables: Vec<VariableDecl>, 
        assertions: Vec<AssertDecl>, 
        conditional_branches: Vec<ComplexIfBranchDecl>, 
        receive_channels: Vec<String> 
    },
    SupervisorDecl { name: String, target_actor: String, strategy: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityLeaseDecl { pub name: String, pub duration_sec: u32, pub quota: u32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDecl { pub name: String, pub unit_type: String, pub literal_value: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertDecl { pub variable: String, pub operator: Operator, pub limit: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition { pub variable: String, pub operator: Operator, pub limit: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexIfBranchDecl { 
    pub cond1: Condition, 
    pub op: LogicalOp, 
    pub cond2: Condition, 
    pub emit_target: String 
}

pub struct Lexer<'a> { chars: std::iter::Peekable<std::str::Chars<'a>> }
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self { Self { chars: input.chars().peekable() } }
    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(&c) = self.chars.peek() { if c.is_whitespace() { self.chars.next(); } else { break; } }
        let ch = self.chars.next()?;
        match ch {
            '{' => Some(Token::OpenBrace), '}' => Some(Token::CloseBrace), ':' => Some(Token::Colon), 
            '=' => Some(Token::Assign), '>' => Some(Token::GreaterThan), '<' => Some(Token::LessThan),
            '@' => Some(Token::At),
            c if c.is_ascii_digit() => {
                let mut num = c.to_string();
                while let Some(&next) = self.chars.peek() { if next.is_ascii_digit() || next == '.' { num.push(self.chars.next().unwrap()); } else { break; } }
                Some(Token::Number(num.parse().unwrap_or(0.0)))
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut ident = c.to_string();
                while let Some(&next) = self.chars.peek() { if next.is_alphanumeric() || next == '_' || next == '/' { ident.push(self.chars.next().unwrap()); } else { break; } }
                match ident.as_str() { 
                    "actor" => Some(Token::Actor), "stream" => Some(Token::Stream), "capability" => Some(Token::Capability), 
                    "requires" => Some(Token::Requires), "state" => Some(Token::State), "receive" => Some(Token::Receive), 
                    "emit" => Some(Token::Emit), "let" => Some(Token::Let), "agent" => Some(Token::Agent), 
                    "model" => Some(Token::Model), "input" => Some(Token::Input), "output" => Some(Token::Output),
                    "assert" => Some(Token::Assert), "if" => Some(Token::If), "else" => Some(Token::Else),
                    "and" => Some(Token::And), "or" => Some(Token::Or), "not" => Some(Token::Not),
                    "supervisor" => Some(Token::Supervisor), "restart" => Some(Token::Restart),
                    "strategy" => Some(Token::Strategy), "ExponentialBackoff" => Some(Token::ExponentialBackoff),
                    "Int64" => Some(Token::Int64), "partition" => Some(Token::Partition), "retention" => Some(Token::Retention),
                    "crdt" => Some(Token::Crdt), "merge" => Some(Token::Merge), "lww" => Some(Token::Lww),
                    "edge" => Some(Token::Edge), "cloud" => Some(Token::Cloud),
                    "lease" => Some(Token::Lease), "expiry" => Some(Token::Expiry), "quota" => Some(Token::Quota),
                    "invariant" => Some(Token::Invariant),
                    "m/s" | "Celsius" | "Volts" | "kg" | "QState" | "GPS" | "HealthScore" | "Synapse" | "ZKProof" | "NodeQuorum" => Some(Token::Unit(ident)), _ => Some(Token::Identifier(ident)) 
                }
            }
            _ => None,
        }
    }
}

pub struct Parser { tokens: Vec<Token>, pos: usize }
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else { None }
    }

    pub fn parse_program(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while self.peek().is_some() {
            if let Some(node) = self.parse_declaration() { nodes.push(node); } else { self.advance(); }
        }
        nodes
    }

    fn parse_declaration(&mut self) -> Option<ASTNode> {
        match self.peek()? {
            Token::Capability => {
                self.advance();
                if let Some(Token::Identifier(name)) = self.advance() { return Some(ASTNode::CapabilityDecl { name }); }
                None
            }
            Token::Stream => {
                self.advance();
                if let Some(Token::Identifier(name)) = self.advance() {
                    let mut p_key = String::new();
                    if Some(Token::OpenBrace) == self.advance() {
                        while let Some(tok) = self.peek() {
                            if *tok == Token::CloseBrace { self.advance(); break; }
                            match self.advance() {
                                Some(Token::Partition) => { if let Some(Token::Identifier(k)) = self.advance() { p_key = k; } }
                                _ => { self.advance(); }
                            }
                        }
                    }
                    return Some(ASTNode::StreamDecl { name, partition_key: p_key, retention_days: 14 });
                }
                None
            }
            Token::Crdt => {
                self.advance();
                if let Some(Token::Identifier(name)) = self.advance() {
                    let mut strategy = String::new();
                    if Some(Token::OpenBrace) == self.advance() {
                        while let Some(tok) = self.peek() {
                            if *tok == Token::CloseBrace { self.advance(); break; }
                            match self.advance() {
                                Some(Token::Merge) => { if let Some(Token::Lww) = self.advance() { strategy = "LWW".to_string(); } }
                                _ => { self.advance(); }
                            }
                        }
                    }
                    return Some(ASTNode::CrdtDecl { name, strategy });
                }
                None
            }
            Token::Invariant => {
                self.advance();
                if let Some(Token::Identifier(actor_target)) = self.advance() {
                    if let Some(Token::Identifier(expr)) = self.advance() {
                        return Some(ASTNode::InvariantDecl { target_actor: actor_target, expression: expr });
                    }
                }
                None
            }
            Token::Supervisor => {
                self.advance();
                if let Some(Token::Identifier(name)) = self.advance() {
                    let mut target = String::new();
                    let mut strategy = String::new();
                    if Some(Token::OpenBrace) == self.advance() {
                        while let Some(tok) = self.peek() {
                            if *tok == Token::CloseBrace { self.advance(); break; }
                            match self.advance() {
                                Some(Token::Restart) => { self.advance(); if let Some(Token::Identifier(t)) = self.advance() { target = t; } }
                                Some(Token::Strategy) => { self.advance(); if let Some(Token::ExponentialBackoff) = self.advance() { strategy = "ExponentialBackoff".to_string(); } }
                                _ => {}
                            }
                        }
                    }
                    return Some(ASTNode::SupervisorDecl { name, target_actor: target, strategy });
                }
                None
            }
            Token::Actor => {
                self.advance();
                if let Some(Token::Identifier(actor_name)) = self.advance() {
                    let mut placement = "local".to_string();
                    let mut requirements = Vec::new();
                    let mut state_variables = Vec::new();
                    let mut assertions = Vec::new();
                    let mut conditional_branches = Vec::new();
                    let mut receive_channels = Vec::new();

                    if self.peek() == Some(&Token::At) {
                        self.advance();
                        match self.advance() {
                            Some(Token::Edge) => placement = "edge".to_string(),
                            Some(Token::Cloud) => placement = "cloud".to_string(),
                            _ => {}
                        }
                    }

                    if Some(Token::OpenBrace) == self.advance() {
                        while let Some(tok) = self.peek() {
                            if *tok == Token::CloseBrace { self.advance(); break; }
                            match self.peek() {
                                Some(Token::Requires) => {
                                    self.advance();
                                    if let Some(Token::Identifier(req_name)) = self.advance() {
                                        let mut duration = 300;
                                        let mut quota = 1000;
                                        if Some(Token::OpenBrace) == self.advance() {
                                            while let Some(l_tok) = self.peek() {
                                                if *l_tok == Token::CloseBrace { self.advance(); break; }
                                                match self.advance() {
                                                    Some(Token::Expiry) => { self.advance(); if let Some(Token::Number(n)) = self.advance() { duration = n as u32; } }
                                                    Some(Token::Quota) => { self.advance(); if let Some(Token::Number(n)) = self.advance() { quota = n as u32; } }
                                                    _ => {}
                                                }
                                            }
                                        }
                                        requirements.push(CapabilityLeaseDecl { name: req_name, duration_sec: duration, quota });
                                    }
                                }
                                Some(Token::Receive) => {
                                    self.advance();
                                    if let Some(Token::Identifier(msg_type)) = self.advance() { receive_channels.push(msg_type); }
                                }
                                Some(Token::State) => {
                                    self.advance();
                                    if Some(Token::OpenBrace) == self.advance() {
                                        while let Some(st) = self.peek() {
                                            if *st == Token::CloseBrace { self.advance(); break; }
                                            if Some(Token::Let) == self.advance() {
                                                if let Some(Token::Identifier(v_name)) = self.advance() {
                                                    if Some(Token::Colon) == self.advance() {
                                                        if let Some(Token::Unit(u_type)) = self.advance() {
                                                            if Some(Token::Assign) == self.advance() {
                                                                 if let Some(Token::Number(val)) = self.advance() {
                                                                    state_variables.push(VariableDecl { name: v_name, unit_type: u_type, literal_value: val });
                                                                 }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else { self.advance(); }
                                        }
                                    }
                                }
                                Some(Token::Assert) => {
                                    self.advance();
                                    if let Some(Token::Identifier(v)) = self.advance() {
                                        let op = match self.advance() {
                                            Some(Token::GreaterThan) => Some(Operator::GreaterThan),
                                            Some(Token::LessThan) => Some(Operator::LessThan),
                                            _ => None,
                                        };
                                        if let Some(actual_op) = op {
                                            if let Some(Token::Number(n)) = self.advance() {
                                                assertions.push(AssertDecl { variable: v, operator: actual_op, limit: n });
                                            }
                                        }
                                    }
                                }
                                Some(Token::If) => {
                                    self.advance();
                                    if let Some(Token::Identifier(v1)) = self.advance() {
                                        let op1 = if self.advance() == Some(Token::GreaterThan) { Operator::GreaterThan } else { Operator::LessThan };
                                        let limit1 = match self.advance() { Some(Token::Number(n)) => n, _ => 0.0 };
                                        let log_op = if self.advance() == Some(Token::And) { LogicalOp::And } else { LogicalOp::Or };
                                        
                                        if let Some(Token::Identifier(v2)) = self.advance() {
                                            let op2 = if self.advance() == Some(Token::GreaterThan) { Operator::GreaterThan } else { Operator::LessThan };
                                            let limit2 = match self.advance() { Some(Token::Number(n)) => n, _ => 0.0 };
                                            
                                            if Some(Token::OpenBrace) == self.advance() {
                                                if Some(Token::Emit) == self.advance() {
                                                    if let Some(Token::Identifier(stream)) = self.advance() {
                                                        conditional_branches.push(ComplexIfBranchDecl {
                                                            cond1: Condition { variable: v1, operator: op1, limit: limit1 },
                                                            op: log_op,
                                                            cond2: Condition { variable: v2, operator: op2, limit: limit2 },
                                                            emit_target: stream
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => { self.advance(); }
                            }
                        }
                    }
                    return Some(ASTNode::ActorDecl { name: actor_name, placement, requirements, state_variables, assertions, conditional_branches, receive_channels });
                }
                None
            }
            _ => None,
        }
    }
}
