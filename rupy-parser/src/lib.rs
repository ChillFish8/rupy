#![allow(unused)]

mod types;

use rustpython_parser::parser;
use rustpython_parser::ast::{
    ExpressionType,
    StatementType,
    Statement,
    Expression,
    ImportSymbol,
    Number,
};

use std::collections::vec_deque::VecDeque;

use crossbeam::queue::SegQueue;
use crossbeam::deque::Worker;

use bytes::Bytes;
use num_bigint::BigInt;

use crate::types::{RawTypes, ParsingTypes};


#[derive(Debug)]
pub enum Instruction {
    Break,
    Continue,
    Pass,
    Return,
    Value(ParsingTypes),
    Import(ImportSymbol),
    AssignVar(Vec<String>),
    Invoke,

}


pub struct AstParser {
    statements: VecDeque<Statement>,
    queue: VecDeque<(usize, Instruction)>,
}

impl AstParser {
    pub fn new() -> Self {
        Self {
            statements: VecDeque::new(),
            queue: VecDeque::new()
        }
    }

    pub fn process_file(&mut self, content: &str) {
        let mut ast = parser::parse_program(content).unwrap();
        ast.statements.reverse();

        for stmt in ast.statements {
            self.statements.push_front(stmt);
        }

        let mut counter = 0usize;
        while let Some(stmt) = self.statements.pop_front() {
            self.handle_stmt(counter, stmt);
            counter += 1;
        }


    }

    pub fn generate_bytecode(&mut self) {
        let mut current_count = 0usize;
        let mut temp_stack = Worker::new_lifo();
        while let Some((id, instr)) = self.queue.pop_front() {
            if current_count != id {
                self.process_stack(&mut temp_stack);
                current_count = id;
            }

            temp_stack.push(instr);
        }
    }

    fn process_stack(&mut self, stack: &mut Worker<Instruction>) {
        while let Some(inst) = stack.pop() {

        }
    }

    fn handle_stmt(&mut self, id: usize, stmt: Statement) {
        match stmt.node {
            StatementType::Break => {
                self.queue.push((id, Instruction::Break));
            },
            StatementType::Continue => {
                self.queue.push((id, Instruction::Continue));
            },
            StatementType::Pass => {
                self.queue.push((id, Instruction::Pass));
            },
            StatementType::Return {
                value,
            } => {
                self.queue.push((id, Instruction::Return));
                if let Some(expr) = value {
                    self.handle_expr_node(id, expr.node);
                } else {
                    self.queue.push((id, Instruction::Value(ParsingTypes::None)));
                }
            },
            StatementType::Import {
                names,
            } => {
                for name in names {
                    self.queue.push((id, Instruction::Import(name)))
                }
            },
            StatementType::ImportFrom {
                level,
                module,
                names,
            } => {
                // todo ???????
            },
            StatementType::Assert {
                test,
                msg,
            } => {

            },
            StatementType::Delete {
                targets,
            } => {

            },
            StatementType::Assign {
                targets,
                value,
            } => {
                let mut vars = vec![];
                for expr in targets {
                    if let ExpressionType::Identifier { name } = expr.node {
                        vars.push(name)
                    } else {
                        unimplemented!()
                    }
                }

                self.queue.push((id, Instruction::AssignVar(vars)));
                self.handle_expr_node(id, value.node);
            },
            StatementType::AugAssign {
                target,
                op,
                value,
            } => {

            },
            StatementType::AnnAssign {
                target,
                annotation,
                value,
            } => {

            }
            StatementType::Expression {
                expression
            } => {
                self.handle_expr_node(id, expression.node);
            },
            StatementType::Global {
                names
            } => {

            },
            StatementType::Nonlocal {
                names,
            } => {

            },
            StatementType::If {
                test,
                body,
                orelse,
            } => {

            },
            StatementType::While {
                test,
                body,
                orelse,
            } => {

            },
            StatementType::Raise {
                exception,
                cause,
            } => {

            },
            StatementType::With {
                is_async,
                items,
                body
            } => {

            },
            StatementType::For {
                is_async,
                target,
                iter,
                body,
                orelse
            } => {

            },
            StatementType::Try {
                body,
                handlers,
                orelse,
                finalbody
            } => {

            },
            StatementType::ClassDef {
                name,
                body,
                bases,
                keywords,
                decorator_list
            } => {

            },
            StatementType::FunctionDef {
                is_async,
                name,
                args,
                body,
                decorator_list,
                returns
            } => {

            },
        }
    }

    fn handle_expr_node(&mut self, id: usize, expr: ExpressionType) {
        match expr {
            ExpressionType::BoolOp { op, values } => {

            },
            ExpressionType::Binop { a, op, b } => {

            },
            ExpressionType::Subscript { a, b } => {

            },
            ExpressionType::Unop { op, a } => {

            },
            ExpressionType::Await { value } => {

            },
            ExpressionType::Yield { value } => {

            },
            ExpressionType::YieldFrom { value } => {

            },
            ExpressionType::Compare { vals, ops } => {

            },
            ExpressionType::Attribute { value, name } => {

            },
            ExpressionType::Call { function, args, keywords } => {
                self.queue.push((id, Instruction::Invoke));
                self.handle_expr_node(id, function.node);

                for expr in args {
                    self.handle_expr_node(id,expr.node);
                }

                for kwarg in keywords {
                    self.queue.push((
                        id,
                        Instruction::Value(ParsingTypes::KeyVar(kwarg.name))
                    ));
                    self.handle_expr_node(id, kwarg.value.node);
                }
            },
            ExpressionType::List { elements } => {

            },
            ExpressionType::Tuple { elements } => {

            },
            ExpressionType::Dict { elements } => {

            },
            ExpressionType::Set { elements } => {

            },
            ExpressionType::Comprehension { kind, generators } => {

            },
            ExpressionType::Starred { value } => {

            },
            ExpressionType::Slice { elements } => {

            },
            ExpressionType::Identifier { name } => {
                self.queue.push((
                    id,
                    Instruction::Value(ParsingTypes::Var(name))
                ))
            },
            ExpressionType::Lambda { args, body } => {

            },
            ExpressionType::IfExpression { test, body, orelse } => {

            },
            ExpressionType::NamedExpression { left, right } => {

            },
            ExpressionType::Number { value } => {
                match value {
                    Number::Integer { value } => {
                        self.queue.push((
                            id,
                            Instruction::Value(ParsingTypes::Int(value)),
                        ))
                    }
                    Number::Float { value } => {
                        self.queue.push((
                            id,
                            Instruction::Value(ParsingTypes::Float(value)),
                        ))
                    },
                    Number::Complex { real, imag } => {
                        self.queue.push((
                            id,
                            Instruction::Value(ParsingTypes::Complex((real, imag))),
                        ))
                    }
                }
            },
            ExpressionType::String { value } => {
                self.queue.push((
                    id,
                    Instruction::Value(ParsingTypes::Str(value)),
                ))
            },
            ExpressionType::Bytes { value } => {
                self.queue.push((
                    id,
                    Instruction::Value(ParsingTypes::Bytes(value)),
                ))
            },
            ExpressionType::True => {
                self.queue.push((
                    id,
                    Instruction::Value(ParsingTypes::True),
                ));
            },
            ExpressionType::False => {
                self.queue.push((
                    id,
                    Instruction::Value(ParsingTypes::False),
                ));
            },
            ExpressionType::None => {
                self.queue.push((
                    id,
                    Instruction::Value(ParsingTypes::None),
                ));
            },
            ExpressionType::Ellipsis => {
                self.queue.push((
                    id,
                    Instruction::Value(ParsingTypes::None),
                ));
            }
        }
    }


}