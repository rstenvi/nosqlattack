

#[derive(Debug, Clone, PartialEq)]
pub enum InjectResult {
    Error,      // DB error
    Delay,      // Time delay
    Length,     // Response length is different
    Header,     // Different header response
    Echo,       // What we send should be printed back to us

    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InjectMethod   {
    All,
    Individual,
    Product,
}

