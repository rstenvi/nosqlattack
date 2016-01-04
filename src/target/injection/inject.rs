
use super::enums;
use super::parameter;
use super::super::web;
use super::super::web::enums::ParameterEncoding;


/// 1 inject attack, used as a template to generate more attacks.
pub struct Template   {
    id: String,
    expected: enums::InjectResult,
    injection: parameter::Parameters,
}


impl Template {
    pub fn new(id: &str, expected: enums::InjectResult, injection: parameter::Parameters) -> Template  {
        Template  {
            id: id.to_string(),
            expected: expected,
            injection: injection,
        }
    }

    pub fn get_id(&self) -> &str  {
        &self.id
    }

    pub fn inject(&mut self, form: &web::Form) -> Option<(Vec<web::Form>, Vec<String>, Vec<String>, enums::InjectResult)>    {
        let supported_enc = self.injection.get_encoding();
        let current_enc = form.get_encoding();
        if supported_enc == ParameterEncoding::UNKNOWN || supported_enc == current_enc    {
            let (forms, replaced, non_match) = self.injection.create_attack(form);
            return Some( (forms, replaced, non_match, self.expected.clone()) );
        }
        None
    }
}



