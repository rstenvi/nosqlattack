
use super::enums;

#[derive(Debug, Clone)]
pub struct Parameter    {
    name: String,
    param_type: enums::ParamType,
    value: String,
}

impl Parameter  {
    pub fn new(name: &str, t: enums::ParamType, value: &str) -> Parameter   {
        Parameter   {
            name: name.to_string(),
            param_type: t,
            value: value.to_string(),
        }
    }

    pub fn get_parameter_url(&self) -> String   {
        format!("{}={}", self.name, self.value)
    }

    pub fn get_parameter_json(&self) -> String {
        let quote_n = match  self.name.find("{")    {
            Some(_) => "",
            None    => "\"",
        };
        let quote_v = match  self.value.find("{")    {
            Some(_) => "",
            None    => "\"",
        };
        format!("{}{}{}:{}{}{}", quote_n, self.name, quote_n, quote_v, self.value, quote_v)
    }


    pub fn set_value(&mut self, val: &str) {
        self.value = val.to_string();
    }
    pub fn set_parameter(&mut self, val: &str) {
        self.name = val.to_string();
    }
    pub fn get_parameter(&self) -> String   {
        self.name.clone()
    }
    pub fn get_value(&self) -> String   {
        self.value.clone()
    }
}


