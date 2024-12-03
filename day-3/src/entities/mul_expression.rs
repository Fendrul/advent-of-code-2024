#[derive(Debug)]
pub struct MulExpression {
    left: usize,
    right: usize,
    instruction: Instruction,
}

impl MulExpression {
    pub fn new(left: usize, right: usize, instruction: Instruction) -> MulExpression {
        MulExpression {
            left,
            right,
            instruction,
        }
    }

    pub fn evaluate(&self) -> usize {
        self.left * self.right
    }

    pub fn evaluate_by_instruction(&self) -> usize {
        match self.instruction {
            Instruction::Dont => 0,
            Instruction::Do => self.evaluate(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Dont,
    Do,
}
