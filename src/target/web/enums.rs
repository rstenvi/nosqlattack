

#[derive(Debug, Clone, PartialEq)]
pub enum ParamType  {
    Text,
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RequestMethod  {
    POST,
    GET,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterEncoding  {
    URL,
    JSON,
    UNKNOWN,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Protocol  {
    HTTP,
    HTTPS,
}


