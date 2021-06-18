pub struct ErrorHandler
{
    had_error: bool,
}

impl ErrorHandler 
{
    pub fn new() -> ErrorHandler
    {
        ErrorHandler
        {
            had_error: false,
        }
    }

    pub fn report(&mut self, line: i32, found_at: String, message: String)
    {
        println!("[line {}] Error {}: {}", line, found_at, message);
        self.had_error = true;
    }

    pub fn error(&mut self, line: i32, message: String)
    {
        self.report(line, String::new(), message);
    }   
}