#[derive(Debug)]
pub struct Calibration {
    result: usize,
    expression: Expression,
}

impl Calibration {
    pub fn new(result: usize, expression: Expression) -> Self {
        Self { result, expression }
    }

    pub fn has_matching_expression_first_part(&self) -> bool {
        self.expression
            .iter_evaluations_first_part()
            .any(|value| value == self.result)
    }

    pub fn has_matching_expression_second_part(&self) -> bool {
        self.expression
            .iter_evaluations_second_part()
            .any(|value| value == self.result)
    }

    pub fn get_expression_iter_first_part(&self) -> Box<dyn Iterator<Item=usize> + '_> {
        self.expression.iter_evaluations_first_part()
    }
    
    pub fn get_expression_iter_second_part(&self) -> Box<dyn Iterator<Item=usize> + '_> {
        self.expression.iter_evaluations_second_part()
    }

    pub fn get_result(&self) -> usize {
        self.result
    }
}

#[derive(Debug)]
pub enum Expression {
    Expr(usize, Box<Expression>),
    Litteral(usize),
}

impl Expression {
    pub fn iter_evaluations_first_part(&self) -> Box<dyn Iterator<Item=usize> + '_> {
        match self {
            Expression::Expr(left, right) => Box::new(
                right
                    .iter_evaluations_first_part()
                    .flat_map(move |right| [left + right, left * right]),
            ),
            Expression::Litteral(value) => Box::new(std::iter::once(*value)),
        }
    }

    pub fn iter_evaluations_second_part(&self) -> Box<dyn Iterator<Item=usize> + '_> {
        match self {
            Expression::Expr(left, right) => {
                Box::new(
                    right
                        .iter_evaluations_second_part()
                        .flat_map(move |right| {
                            [left + right, left * right, concat_numbers(right, *left)]
                        }))
            }
            Expression::Litteral(value) => Box::new(std::iter::once(*value)),
        }
    }
}

fn concat_numbers(a: usize, b: usize) -> usize {
    format!("{a}{b}").parse().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_concat_numbers() {
        assert_eq!(concat_numbers(1, 2), 12);
        assert_eq!(concat_numbers(12, 3), 123);
        assert_eq!(concat_numbers(123, 4), 1234);
        assert_eq!(concat_numbers(1234, 5), 12345);
    }
}
