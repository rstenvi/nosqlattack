/// Inject module, all things related to crafting injection parameters and evaluting differences
/// between results.


mod enums;
mod inject;
mod parameter;
mod response;

use ini::Ini;
use super::web;
use super::web::enums::ParameterEncoding;
use time::PreciseTime;
use std::io::Read;


/// Holds information about all the injection attacks.
pub struct Inject    {
    attacks: Vec<inject::Template>,
}

impl Inject  {
    pub fn new(fname: &str) -> Inject   {
        let mut attacks: Vec<inject::Template> = Vec::new();


        // Read the configuration file
        let conf = Ini::load_from_file(fname).unwrap();
        let mut sections = 0;
        for (sec, prop) in conf.iter() {
            let id: String = sec.clone().unwrap();
            let mut inj_res: enums::InjectResult = enums::InjectResult::Unknown;
            let mut param_template: String = "".to_string();
            let mut value_template: String = "".to_string();
            let mut inj_in: enums::InjectMethod = enums::InjectMethod::All;
            let mut valid_enc: ParameterEncoding = ParameterEncoding::UNKNOWN;

            for (key, value) in prop.iter() {
                match key.as_ref()   {
                    "result" => {
                        inj_res = match value.as_ref()    {
                            "length" => enums::InjectResult::Length,
                            "echo"   => enums::InjectResult::Echo,
                            "error"  => enums::InjectResult::Error,
                            "delay"  => enums::InjectResult::Delay,
                            _        => panic!("Not supported"),
                        }
                    }
                    "paramname" => {
                        param_template = value.to_string();
                    }
                    "paramvalue" => {
                        value_template = value.to_string();
                    }
                    "injectparam" => {
                        inj_in = match value.as_ref()    {
                            "all"        => enums::InjectMethod::All,
                            "individual" => enums::InjectMethod::Individual,
                            _            => panic!("Not supported"),
                        }
                    }
                    "validenc" => {
                        valid_enc = match value.as_ref()    {
                            "URL"  => ParameterEncoding::URL,
                            "JSON" => ParameterEncoding::JSON,
                            _      => panic!("Not supported"),
                        }
                    }
                    _ => {
                    }
                }
            }

            sections += 1;
            // Push the new element on the vector
            attacks.push(
                inject::Template::new(
                    &id, 
                    inj_res, 
                    parameter::Parameters::new(
                        param_template, 
                        value_template, 
                        inj_in, 
                        valid_enc
                    )
                )
            );


        }

        info!("Found {} injection methods", sections);
        assert!(sections > 0, "Unable to find any injection parameters, did you supply the correct file?");

        Inject   {
            attacks: attacks,
        }
    }

    pub fn run_inject_test(&self, form: &mut web::Form) -> response::InjResponse  {
        let start = PreciseTime::now();
        debug!("Sending: {}", form.get_curl());
        let mut res = form.send_request();
        let end = PreciseTime::now();

        let req_time = start.to(end);
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();
        debug!("RESPONSE: {}", s);

        response::InjResponse::new(req_time, res.status, &s, &res.headers)
    }

    pub fn inject_test_all_form(&mut self, form: &mut web::Form, base: &response::InjResponse) -> Vec<u32>    {
        let mut ret: Vec<u32> = Vec::new();
        loop    {
            // Go until there are no more attacks left
            let mut attack = match self.attacks.pop()   {
                Some(s) => s,
                None    => break,
            };

            // If we don't get a valid form we jump to the next one
            let (mut web_form, replaced) = match attack.inject(form)  {
                Some(s) => s,
                None    => continue,
            };

            // We now have a list of web forms we can inject
            info!("Using attack with ID {}, {} tests", attack.get_id(), web_form.len());
            loop    {
                let mut web = match web_form.pop()  {
                    Some(w) => w,
                    None    => break,
                };
                let response = self.run_inject_test(&mut web);
                let cmp_res = base.cmp(&response, replaced.clone());
                if cmp_res > 0  {
                    println!("Found a possible attack ({}): {}", cmp_res, web.get_curl());
                }
                ret.push(cmp_res as u32)
            }

        }
        ret
    }
}

