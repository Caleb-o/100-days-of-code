// Temporary implementation of Value
pub type Value = f64;

pub struct ValueArray
{
    pub(super) values: Vec<Value>,
}

impl ValueArray
{
    pub fn new() -> ValueArray
    {
        ValueArray
        {
            values: Vec::new(),
        }
    }

    pub fn write(&mut self, value: Value)
    {
        self.values.push(value);
    }

    pub fn free(&mut self)
    {
        self.values.clear();
    }
}

pub fn print_value(value: Value)
{
    print!("{}", value);
}