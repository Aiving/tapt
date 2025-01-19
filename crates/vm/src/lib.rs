mod chunk;
mod op;
mod value;

pub use self::{chunk::Chunk, op::OpCode, value::*};
use std::any::Any;
use tapt_parser::prelude::Operator;

#[derive(Debug, Default)]
pub struct StackFrame {
    position: usize,
    stack_position: usize,
    returned: Option<Value>,
    slots: Vec<Value>,
}

impl StackFrame {
    #[must_use]
    pub const fn new(stack_position: usize) -> Self {
        Self {
            position: 0,
            stack_position,
            slots: Vec::new(),
            returned: None,
        }
    }

    // pub fn get_var(&self, index: usize) -> &Value {
    //     let Value::Object(name) = self.get_const(index) else {
    //         unreachable!()
    //     };
    //     let Object::String(name) = &*name.borrow();

    //     &self.variables[name]
    // }

    // pub fn set_var(&mut self, index: usize, value: Value) {
    //     let Value::Object(name) = self.get_const(index) else {
    //         unreachable!()
    //     };
    //     let Object::String(name) = (*name.borrow()).clone();

    //     if let Some(variable) = self.variables.get_mut(&name) {
    //         *variable = value;
    //     } else {
    //         self.variables.insert(name, value);
    //     }
    // }

    #[must_use]
    pub fn get_slot(&self, slot: usize) -> Value {
        self.slots[slot].clone()
    }

    pub fn set_slot(&mut self, slot: usize, value: Value) {
        if self.slots.is_empty() || self.slots.len() < slot + 1 {
            self.slots.resize(slot + 1, Value::None);
        }

        self.slots[slot] = value;
    }

    pub fn reset(&mut self) {
        self.slots.clear();
        self.position = 0;
    }
}

#[derive(Debug)]
pub struct VM {
    pub state: Box<dyn Any>,
    pub is_running: bool,
    pub position: usize,
    pub stack: Vec<Value>,
    pub frames: Vec<StackFrame>,
}

impl VM {
    #[must_use]
    pub fn new(state: impl Any) -> Self {
        Self {
            state: Box::new(state),
            position: 0,
            is_running: false,
            stack: Vec::new(),
            frames: vec![StackFrame::new(0)],
        }
    }

    #[must_use]
    pub fn frame(&self) -> &StackFrame {
        let frame = self.frames.len() - 1;

        &self.frames[frame]
    }

    pub fn frame_mut(&mut self) -> &mut StackFrame {
        let frame = self.frames.len() - 1;

        &mut self.frames[frame]
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        let Some(value) = self.stack.pop() else {
            unreachable!()
        };

        value
    }

    #[must_use]
    pub fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack.len() - 1 - distance].clone()
    }

    pub fn reset(&mut self) {
        self.frame_mut().reset();
        self.position = 0;
        self.is_running = false;
    }

    pub fn push_frame(&mut self) {
        self.frames.push(StackFrame::new(self.stack.len()));
    }

    pub fn pop_frame(&mut self) -> Option<Value> {
        let Some(frame) = self.frames.pop() else {
            unreachable!()
        };

        self.stack.truncate(frame.stack_position);

        frame.returned
    }

    fn binary_op(&mut self, operator: &Operator) {
        let b = self.pop();
        let a = self.pop();

        match operator {
            Operator::Equal => self.push(Value::Boolean(a == b)),
            Operator::GreaterThan => self.push(Value::Boolean(a > b)),
            Operator::LessThan => self.push(Value::Boolean(a < b)),
            operator => match (a, b) {
                (Value::Integer(a), Value::Integer(b)) => {
                    let value = match operator {
                        Operator::Add => a + b,
                        Operator::Sub => a - b,
                        Operator::Mul => a * b,
                        Operator::Div => a / b,
                        _ => unreachable!(),
                    };

                    self.push(Value::Integer(value));
                }
                (Value::Float(a), Value::Float(b)) => {
                    let value = match operator {
                        Operator::Add => a + b,
                        Operator::Sub => a - b,
                        Operator::Mul => a * b,
                        Operator::Div => a / b,
                        _ => unreachable!(),
                    };

                    self.push(Value::Float(value));
                }
                (a, b) => {
                    panic!("SUKA NELZYA {a:?} {operator} {b:?}");
                }
            },
        }
    }

    fn call(&mut self, func: &Function, args: usize) {
        let position = self.position;

        let mut args = self.stack.split_off(self.stack.len() - args);

        args.push(self.pop());

        let mut frame = StackFrame::new(self.stack.len());

        frame.slots = args;

        self.frames.push(frame);

        self.interpret(&func.chunk);

        if let Some(returned) = self.frames.pop().and_then(|frame| frame.returned) {
            self.push(returned);
        }

        self.is_running = true;
        self.position = position;
    }

    /// # Panics
    ///
    /// Panics if negating value which is not number or boolean
    pub fn interpret(&mut self, chunk: &Chunk) -> Value {
        self.position = 0;
        self.is_running = true;

        while self.is_running {
            let (_, instruction) = &chunk[self.position];

            match instruction {
                OpCode::LoadConst(value) => {
                    self.push(chunk.get_const_cloned(*value));
                }
                OpCode::Copy => self.push(self.peek(0)),
                OpCode::PushFrame => self.push_frame(),
                OpCode::PopFrame => {
                    if let Some(value) = self.pop_frame() {
                        self.push(value);
                    }
                }
                OpCode::GetLocal(frame, slot) => {
                    let value = self.frames[(self.frames.len() as isize - 1 + *frame) as usize]
                        .get_slot(*slot);

                    self.push(value);
                }
                OpCode::SetLocal(frame, slot) => {
                    let value = self.pop();

                    if let Some(frame) = frame {
                        self.frames[*frame].set_slot(*slot, value);
                    } else {
                        self.frame_mut().set_slot(*slot, value);
                    }
                }
                OpCode::Equal => self.binary_op(&Operator::Equal),
                OpCode::Greater => self.binary_op(&Operator::GreaterThan),
                OpCode::Less => self.binary_op(&Operator::LessThan),
                OpCode::Add => self.binary_op(&Operator::Add),
                OpCode::Sub => self.binary_op(&Operator::Sub),
                OpCode::Mul => self.binary_op(&Operator::Mul),
                OpCode::Div => self.binary_op(&Operator::Div),
                OpCode::Negate => match self.pop() {
                    Value::Integer(value) => self.push(Value::Integer(-value)),
                    Value::Float(value) => self.push(Value::Float(-value)),
                    Value::Boolean(value) => self.push(Value::Boolean(!value)),
                    _ => panic!("SUKA TAK NELZYA"),
                },
                OpCode::Return => {
                    if self.stack.len() > self.frame().stack_position {
                        self.frame_mut().returned = Some(self.pop());
                    }
                }
                OpCode::Halt => self.is_running = false,
                OpCode::Jump(offset) => {
                    let unsigned_offset = offset.unsigned_abs();

                    if *offset >= 0 {
                        self.position += unsigned_offset;
                    } else {
                        self.position -= unsigned_offset;
                    }
                }
                OpCode::JumpIfFalse(offset) => {
                    if matches!(self.peek(0), Value::Boolean(_))
                        && self.pop() == Value::Boolean(false)
                    {
                        self.position += *offset;
                    }
                }
                OpCode::Pop => {
                    self.pop();
                }
                OpCode::Call(args) => {
                    if let Value::Object(func) = self.peek(*args) {
                        if let Object::Function(func) = &*func.borrow() {
                            self.call(func, *args);
                        } else if let Object::NativeFunction(func) = &*func.borrow() {
                            let args = self.stack.split_off(self.stack.len() - *args);

                            self.pop();

                            let returned = (func.func)(self, Args {
                                args: args.into_iter(),
                            });

                            if returned != Value::None {
                                self.push(returned);
                            }
                        }
                    }
                }
                OpCode::CreateInstance => {
                    let Value::Object(value) = self.pop() else {
                        unreachable!()
                    };

                    if let Object::Struct(value) = &*value.borrow() {
                        let values = self.stack.split_off(self.stack.len() - value.fields.len());

                        self.push(Value::object(Object::StructInstance(StructInstance {
                            name: value.name.clone(),
                            fields: values
                                .into_iter()
                                .zip(&value.fields)
                                .map(|(value, (name, _))| (name.clone(), value))
                                .collect(),
                        })));
                    } else if let Object::Record(value) = &*value.borrow() {
                        let values = self.stack.split_off(self.stack.len() - value.fields.len());

                        self.push(Value::object(Object::RecordInstance(RecordInstance {
                            name: value.name.clone(),
                            fields: values,
                        })));
                    } else {
                        unreachable!()
                    };
                }
                OpCode::GetProperty(prop) => {
                    let value = self.pop();

                    if let Value::Object(object) = value {
                        if let Object::StructInstance(value) = &*object.borrow() {
                            self.push(value.fields[*prop].1.clone());
                        } else if let Object::RecordInstance(value) = &*object.borrow() {
                            self.push(value.fields[*prop].clone());
                        }
                    }
                }
                OpCode::SetProperty(prop) => {
                    let property_value = self.pop();
                    let value = self.pop();

                    if let Value::Object(object) = value {
                        if let Object::StructInstance(value) = &mut *object.borrow_mut() {
                            value.fields[*prop].1 = property_value;
                        } else if let Object::RecordInstance(value) = &mut *object.borrow_mut() {
                            value.fields[*prop] = property_value;
                        }
                    }
                }
            }

            self.position += 1;
        }

        self.frame_mut().returned.take().unwrap_or(Value::None)
    }
}
