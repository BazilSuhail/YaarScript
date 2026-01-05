use crate::core::token::TokenType;
use crate::ir_pipeline::tac::{Instruction, Operand};
use std::fs;
use std::collections::{HashSet, HashMap};

pub struct IROptimizer {
    instructions: Vec<Instruction>,
}

impl IROptimizer {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    pub fn run(&mut self) {
        let mut modified = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 10;

        // Fixed-point iteration: run until no more optimizations can be made
        while modified && iterations < MAX_ITERATIONS {
            modified = false;
            iterations += 1;

            modified |= self.constant_folding();
            modified |= self.constant_propagation();
            modified |= self.copy_propagation();
            modified |= self.peephole_optimization();
            modified |= self.dead_code_elimination_pass();
        }
    }

    pub fn get_instructions(&self) -> Vec<Instruction> {
        self.instructions.clone()
    }

    /// Evaluates expressions with constants (Int, Float, Bool) at compile time.
    fn constant_folding(&mut self) -> bool {
        let mut modified = false;
        for i in 0..self.instructions.len() {
            let new_instr = match &self.instructions[i] {
                Instruction::Binary(dest, op, l, r) => {
                    if let Some(res_op) = self.evaluate_binary(l, op, r) {
                        modified = true;
                        Some(Instruction::Assign(dest.clone(), res_op))
                    } else {
                        None
                    }
                }
                Instruction::Unary(dest, op, src) => {
                    if let Some(res_op) = self.evaluate_unary(op, src) {
                        modified = true;
                        Some(Instruction::Assign(dest.clone(), res_op))
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if let Some(instr) = new_instr {
                self.instructions[i] = instr;
            }
        }
        modified
    }

    fn evaluate_binary(&self, l: &Operand, op: &TokenType, r: &Operand) -> Option<Operand> {
        match (l, r) {
            (Operand::Int(lv), Operand::Int(rv)) => match op {
                TokenType::Plus => Some(Operand::Int(lv + rv)),
                TokenType::Minus => Some(Operand::Int(lv - rv)),
                TokenType::Multiply => Some(Operand::Int(lv * rv)),
                TokenType::Divide if *rv != 0 => Some(Operand::Int(lv / rv)),
                TokenType::Modulo if *rv != 0 => Some(Operand::Int(lv % rv)),
                TokenType::EqualOp => Some(Operand::Bool(lv == rv)),
                TokenType::Ne => Some(Operand::Bool(lv != rv)),
                TokenType::Lt => Some(Operand::Bool(lv < rv)),
                TokenType::Gt => Some(Operand::Bool(lv > rv)),
                TokenType::Le => Some(Operand::Bool(lv <= rv)),
                TokenType::Ge => Some(Operand::Bool(lv >= rv)),
                TokenType::Power => {
                    if *rv < 0 { None }
                    else { Some(Operand::Int(lv.pow(*rv as u32))) }
                }
                _ => None,
            },
            (Operand::Float(lv), Operand::Float(rv)) => match op {
                TokenType::Plus => Some(Operand::Float(lv + rv)),
                TokenType::Minus => Some(Operand::Float(lv - rv)),
                TokenType::Multiply => Some(Operand::Float(lv * rv)),
                TokenType::Divide if *rv != 0.0 => Some(Operand::Float(lv / rv)),
                TokenType::EqualOp => Some(Operand::Bool(lv == rv)),
                TokenType::Ne => Some(Operand::Bool(lv != rv)),
                TokenType::Lt => Some(Operand::Bool(lv < rv)),
                TokenType::Gt => Some(Operand::Bool(lv > rv)),
                TokenType::Power => Some(Operand::Float(lv.powf(*rv))),
                _ => None,
            },
            (Operand::Bool(lv), Operand::Bool(rv)) => match op {
                TokenType::And => Some(Operand::Bool(*lv && *rv)),
                TokenType::Or => Some(Operand::Bool(*lv || *rv)),
                TokenType::EqualOp => Some(Operand::Bool(lv == rv)),
                TokenType::Ne => Some(Operand::Bool(lv != rv)),
                _ => None,
            },
            _ => None,
        }
    }

    fn evaluate_unary(&self, op: &TokenType, src: &Operand) -> Option<Operand> {
        match src {
            Operand::Int(v) if *op == TokenType::Minus => Some(Operand::Int(-v)),
            Operand::Float(v) if *op == TokenType::Minus => Some(Operand::Float(-v)),
            Operand::Bool(v) if *op == TokenType::Not => Some(Operand::Bool(!v)),
            _ => None,
        }
    }

    /// Tracks variables assigned to constants and replaces their uses.
    fn constant_propagation(&mut self) -> bool {
        let mut modified = false;
        let mut constants: HashMap<String, Operand> = HashMap::new();
        let mut immutable_vars: HashSet<String> = HashSet::new();
        let mut global_vars: HashSet<String> = HashSet::new();
        let mut function_depth: usize = 0;

        // Pre-scan for global constants
        for instr in &self.instructions {
            match instr {
                Instruction::FuncStart(_, _, _) => break,
                Instruction::Declare(type_str, name, init) => {
                    global_vars.insert(name.clone());
                    if type_str.contains("const") {
                        immutable_vars.insert(name.clone());
                        if let Some(val) = init {
                            if Self::is_literal(val) {
                                constants.insert(name.clone(), val.clone());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        for instr in &mut self.instructions {
            match instr {
                Instruction::Declare(type_str, name, init) => {
                    let is_const = type_str.contains("const");
                    let is_global = function_depth == 0;

                    if is_const { immutable_vars.insert(name.clone()); }
                    if is_global { global_vars.insert(name.clone()); }

                    if let Some(val) = init {
                        modified |= Self::replace_operand(val, &constants);
                        if Self::is_literal(val) && is_const {
                            constants.insert(name.clone(), val.clone());
                        } else if !is_const || !is_global {
                            constants.remove(name);
                        }
                    } else if !is_const || !is_global {
                        constants.remove(name);
                    }
                }
                Instruction::Assign(dest, src) => {
                    modified |= Self::replace_operand(src, &constants);
                    if let Some(dest_key) = Self::get_key(dest) {
                        if Self::is_literal(src) {
                            constants.insert(dest_key, src.clone());
                        } else {
                            constants.remove(&dest_key);
                            immutable_vars.remove(&dest_key);
                        }
                    }
                }
                Instruction::Binary(dest, op, l, r) => {
                    modified |= Self::replace_operand(l, &constants);
                    modified |= Self::replace_operand(r, &constants);
                    if let Some(dk) = Self::get_key(dest) {
                        constants.remove(&dk);
                        immutable_vars.remove(&dk);
                    }
                    if *op == TokenType::AssignOp {
                        if let Some(lk) = Self::get_key(l) {
                            constants.remove(&lk);
                            immutable_vars.remove(&lk);
                        }
                    }
                }
                Instruction::Unary(dest, _, src) => {
                    modified |= Self::replace_operand(src, &constants);
                    if let Some(dk) = Self::get_key(dest) {
                        constants.remove(&dk);
                        immutable_vars.remove(&dk);
                    }
                }
                Instruction::IfTrue(cond, _) | Instruction::IfFalse(cond, _) => {
                    modified |= Self::replace_operand(cond, &constants);
                }
                Instruction::Param(p) => {
                    modified |= Self::replace_operand(p, &constants);
                }
                Instruction::Call(dest, _, _) => {
                    if let Some(d) = dest {
                        if let Some(dk) = Self::get_key(d) {
                            constants.remove(&dk);
                            immutable_vars.remove(&dk);
                        }
                    }
                }
                Instruction::Random(dest, min, max) => {
                    modified |= Self::replace_operand(min, &constants);
                    modified |= Self::replace_operand(max, &constants);
                    if let Some(dk) = Self::get_key(dest) {
                        constants.remove(&dk);
                        immutable_vars.remove(&dk);
                    }
                }
                Instruction::Read(dest) | Instruction::Time(dest) => {
                    if let Some(dk) = Self::get_key(dest) {
                        constants.remove(&dk);
                        immutable_vars.remove(&dk);
                    }
                }
                Instruction::Return(val) => {
                    if let Some(v) = val {
                        modified |= Self::replace_operand(v, &constants);
                    }
                }
                Instruction::Print(args) => {
                    for arg in args {
                        modified |= Self::replace_operand(arg, &constants);
                    }
                }
                Instruction::Label(_) => {
                    constants.retain(|name, _| global_vars.contains(name) && immutable_vars.contains(name));
                }
                Instruction::FuncStart(_, _, _) => {
                    function_depth += 1;
                    constants.retain(|name, _| global_vars.contains(name) && immutable_vars.contains(name));
                    immutable_vars.retain(|name| global_vars.contains(name));
                }
                Instruction::FuncEnd => {
                    function_depth = function_depth.saturating_sub(1);
                }
                _ => {}
            }
        }
        modified
    }

    fn copy_propagation(&mut self) -> bool {
        let mut modified = false;
        let mut copies: HashMap<String, Operand> = HashMap::new();

        for instr in &mut self.instructions {
            match instr {
                Instruction::Declare(_, name, init) => {
                    if let Some(val) = init {
                        modified |= Self::replace_operand(val, &copies);
                        if Self::get_key(val).is_some() {
                            copies.insert(name.clone(), val.clone());
                        }
                    }
                    copies.retain(|_, v| Self::get_key(v) != Some(name.clone()));
                }
                Instruction::Assign(dest, src) => {
                    modified |= Self::replace_operand(src, &copies);
                    if let Some(d_key) = Self::get_key(dest) {
                        if Self::get_key(src).is_some() {
                            copies.insert(d_key.clone(), src.clone());
                        } else {
                            copies.remove(&d_key);
                        }
                        copies.retain(|_, v| Self::get_key(v) != Some(d_key.clone()));
                    }
                }
                Instruction::Binary(dest, op, l, r) => {
                    modified |= Self::replace_operand(l, &copies);
                    modified |= Self::replace_operand(r, &copies);
                    if let Some(dk) = Self::get_key(dest) {
                        copies.remove(&dk);
                        copies.retain(|_, v| Self::get_key(v) != Some(dk.clone()));
                    }
                    if *op == TokenType::AssignOp {
                        if let Some(lk) = Self::get_key(l) {
                            copies.remove(&lk);
                            copies.retain(|_, v| Self::get_key(v) != Some(lk.clone()));
                        }
                    }
                }
                Instruction::Unary(dest, _, src) => {
                    modified |= Self::replace_operand(src, &copies);
                    if let Some(dk) = Self::get_key(dest) {
                        copies.remove(&dk);
                        copies.retain(|_, v| Self::get_key(v) != Some(dk.clone()));
                    }
                }
                Instruction::IfTrue(cond, _) | Instruction::IfFalse(cond, _) | Instruction::Param(cond) => {
                    modified |= Self::replace_operand(cond, &copies);
                }
                Instruction::Return(val) => {
                    if let Some(v) = val { modified |= Self::replace_operand(v, &copies); }
                }
                Instruction::Print(args) => {
                    for arg in args { modified |= Self::replace_operand(arg, &copies); }
                }
                Instruction::Call(dest, _, _) => {
                    if let Some(d) = dest {
                        if let Some(dk) = Self::get_key(d) {
                            copies.remove(&dk);
                            copies.retain(|_, v| Self::get_key(v) != Some(dk.clone()));
                        }
                    }
                }
                Instruction::Random(dest, min, max) => {
                    modified |= Self::replace_operand(min, &copies);
                    modified |= Self::replace_operand(max, &copies);
                    if let Some(dk) = Self::get_key(dest) {
                        copies.remove(&dk);
                        copies.retain(|_, v| Self::get_key(v) != Some(dk.clone()));
                    }
                }
                Instruction::Time(dest) | Instruction::Read(dest) => {
                    if let Some(dk) = Self::get_key(dest) {
                        copies.remove(&dk);
                        copies.retain(|_, v| Self::get_key(v) != Some(dk.clone()));
                    }
                }
                Instruction::Label(_) | Instruction::FuncStart(_, _, _) => { copies.clear(); }
                _ => {}
            }
        }
        modified
    }

    fn dead_code_elimination_pass(&mut self) -> bool {
        let mut used = HashSet::new();
        // Mark pass
        for instr in &self.instructions {
            match instr {
                Instruction::Declare(_, _, init) => { if let Some(op) = init { Self::mark_used(op, &mut used); } }
                Instruction::Assign(_, src) | Instruction::Unary(_, _, src) => Self::mark_used(src, &mut used),
                Instruction::Binary(_, _, l, r) => { Self::mark_used(l, &mut used); Self::mark_used(r, &mut used); }
                Instruction::IfTrue(cond, _) | Instruction::IfFalse(cond, _) | Instruction::Param(cond) => Self::mark_used(cond, &mut used),
                Instruction::Random(_, min, max) => { Self::mark_used(min, &mut used); Self::mark_used(max, &mut used); }
                Instruction::Return(val) => { if let Some(v) = val { Self::mark_used(v, &mut used); } }
                Instruction::Print(args) => { for arg in args { Self::mark_used(arg, &mut used); } }
                _ => {}
            }
        }

        // Sweep pass
        let mut i = 0;
        let mut modified = false;
        while i < self.instructions.len() {
            let should_remove = match &self.instructions[i] {
                Instruction::Assign(dest, _) | Instruction::Unary(dest, _, _) => {
                    Self::get_key(dest).map_or(false, |key| !used.contains(&key))
                }
                Instruction::Binary(dest, op, _, _) => {
                    if *op == TokenType::AssignOp { false }
                    else { Self::get_key(dest).map_or(false, |key| !used.contains(&key)) }
                }
                Instruction::Random(dest, _, _) | Instruction::Time(dest) => {
                    Self::get_key(dest).map_or(false, |key| !used.contains(&key))
                }
                Instruction::Declare(_, name, _) => !used.contains(name),
                _ => false,
            };

            if should_remove {
                self.instructions.remove(i);
                modified = true;
            } else {
                i += 1;
            }
        }
        modified
    }

    fn peephole_optimization(&mut self) -> bool {
        let mut modified = false;
        let mut i = 0;
        while i < self.instructions.len() {
            let mut advance = true;

            // Dead Branch Elimination
            let replacement = match &self.instructions[i] {
                Instruction::IfTrue(cond, lbl) => {
                    match cond {
                        Operand::Bool(true) => Some(Instruction::Goto(lbl.clone())),
                        Operand::Bool(false) => Some(Instruction::Comment("Removed ifTrue(false)".into())),
                        _ => None,
                    }
                }
                Instruction::IfFalse(cond, lbl) => {
                    match cond {
                        Operand::Bool(true) => Some(Instruction::Comment("Removed ifFalse(true)".into())),
                        Operand::Bool(false) => Some(Instruction::Goto(lbl.clone())),
                        _ => None,
                    }
                }
                Instruction::Binary(dest, op, l, r) => {
                    // Algebraic Identities
                    match (op, l, r) {
                        (TokenType::Plus, val, Operand::Int(0)) | (TokenType::Plus, Operand::Int(0), val) => Some(Instruction::Assign(dest.clone(), val.clone())),
                        (TokenType::Minus, val, Operand::Int(0)) => Some(Instruction::Assign(dest.clone(), val.clone())),
                        (TokenType::Multiply, val, Operand::Int(1)) | (TokenType::Multiply, Operand::Int(1), val) => Some(Instruction::Assign(dest.clone(), val.clone())),
                        (TokenType::Multiply, _, Operand::Int(0)) | (TokenType::Multiply, Operand::Int(0), _) => Some(Instruction::Assign(dest.clone(), Operand::Int(0))),
                        _ => None,
                    }
                }
                _ => None,
            };

            if let Some(instr) = replacement {
                self.instructions[i] = instr;
                modified = true;
            }

            // Redundant Jump Elimination
            if i + 1 < self.instructions.len() {
                if let Instruction::Goto(target) = &self.instructions[i] {
                    if let Instruction::Label(label_name) = &self.instructions[i+1] {
                        if target == label_name {
                            self.instructions.remove(i);
                            modified = true;
                            advance = false;
                        }
                    }
                }
            }

            if advance { i += 1; }
        }
        modified
    }

    fn get_key(op: &Operand) -> Option<String> {
        match op {
            Operand::Var(name) => Some(name.clone()),
            Operand::Temp(id) => Some(format!("t{}", id)),
            _ => None,
        }
    }

    fn is_literal(op: &Operand) -> bool {
        matches!(op, Operand::Int(_) | Operand::Float(_) | Operand::Bool(_) | Operand::Char(_) | Operand::String(_))
    }

    fn mark_used(op: &Operand, set: &mut HashSet<String>) {
        if let Some(key) = Self::get_key(op) { set.insert(key); }
    }

    fn replace_operand(op: &mut Operand, map: &HashMap<String, Operand>) -> bool {
        if let Some(key) = Self::get_key(op) {
            if let Some(replacement) = map.get(&key) {
                *op = replacement.clone();
                return true;
            }
        }
        false
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut content = String::new();
        for instr in &self.instructions { content.push_str(&format!("{}\n", instr)); }
        fs::write(filename, content)
    }
}