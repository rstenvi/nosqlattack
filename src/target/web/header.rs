


#[derive(Debug, Clone)]
pub struct WebHeader    {
    name: String,
    value: String,
}

impl WebHeader {
    /*
     * Never used, but OK to have.
    pub fn new(name: &str, value: &str) -> WebHeader {
        WebHeader   {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
    */

    pub fn create_string(whole: &str) -> WebHeader   {
        let vec: Vec<&str> = whole.split(":").collect();
        assert_eq!(vec.len(), 2);
        
        WebHeader   {
            name: vec[0].to_string(),
            value: vec[1].to_string(),
        }
    }

    /*
    pub fn get_header(&self) -> String  {
        format!("{}: {}", self.name, self.value)
    }
    */

    pub fn get_name(&self) -> String    { self.name.clone() }
    pub fn get_value(&self) -> String    { self.value.clone() }
}

