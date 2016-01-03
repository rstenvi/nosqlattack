
use super::enums;

use super::super::web;
use super::super::web::enums::ParameterEncoding;
use std::io::Read;
use rand::{thread_rng, Rng};

pub struct Parameters  {
    param_template: String,
    val_template: String,
    inj_in: enums::InjectMethod,
    valid_enc: ParameterEncoding,
}

impl Parameters    {
    pub fn new(param_template: String, val_template: String, inj_in: enums::InjectMethod, v_enc: ParameterEncoding) -> Parameters {
        Parameters {
            param_template: param_template,
            val_template: val_template,
            inj_in: inj_in,
            valid_enc: v_enc,
        }
    }

    pub fn get_encoding(&self) -> ParameterEncoding    {
        self.valid_enc.clone()
    }

    fn create_strings(&mut self, template: String, default: String) -> (Vec<String>, Vec<String>) {
        let mut ret = Vec::new();
        let mut replaced = Vec::new();

        let mut tl = template.clone();

        if tl.is_empty()    {
            ret.push("".to_string());
            return (ret, replaced);
        }

        match tl.find("$default")   {
            Some(_) => {
                tl = str::replace(&tl, "$default", &default);
            }
            None => {}
        }

        match tl.find("$rand")  {
            Some(_) => {
                let rand: String = thread_rng().gen_ascii_chars().take(10).collect();
                tl = str::replace(&tl, "$rand", &rand);
                replaced.push(rand.clone());
            }
            None => {}
        }
        match tl.find("$jsdelay")   {
            Some(_) => {
                tl = str::replace(&tl, "$jsdelay",
                "var cd;var d=new Date();do{cd=new Date();}while(cd-d<10000)");
            }
            None => {}
        }

        // Now we are done with all the single replacements. Now we can check for "||"

        let vec: Vec<&str> = tl.split("||").collect();
        for v in vec    {
            ret.push(v.to_string().trim().to_string());
        }
        
        // Any changes we make now, we must make in all the strings and if we need multiple
        // substitutions we must insert new ones for each of the ones we have in "ret"
        // 
        // Here we are going to replace "$endquote" and "$beginquote". This is used when we need to
        // end something in the code we are injecting, but we don'e know how it start or how we
        // should end it. It could for example start with double quotes, single quotes or no
        // quotes. In more complex scenarios it can start with <("> and we must therefore end our
        // initial variable with <")> and then use the same opening at the end to make it valid JS.
        // This code will ensure that we enumerate the most common options and ensure that they are
        // consistent, i.e. opening and closing quotes match.
        //
        // Some care should be used when creating the rules in *.ini file as this will quickly grow
        // into many statements.
        //
        // endquote and beginquote will also appear out of order, since we need an endquote first to
        // close the initial code and a beginquote at the end to open 
        //
        // The following rules we use:
        // - There are 3 different quotes, <'>, <"> and <> (no quotes), each quote is used for each
        //   possible opening and closing.
        // - Opening: <(> 
        //
        // TODO: Don't need to create this if we have no quotes to replace
        // TODO: Should be relatively easy to add one to replace "$quote"
        

        // First we create all the possible quote combinations
        let quotes   = vec!["", "'", "\""];
        let startend = vec![("(", ")")];
        let mut beginquotes: Vec<String> = Vec::new();
        let mut endquotes: Vec<String>   = Vec::new();
        for (i, _) in quotes.iter().enumerate()   {
            beginquotes.push(quotes[i].to_string());
            endquotes.push(quotes[i].to_string());
            for (j, _) in startend.iter().enumerate() {
                beginquotes.push(quotes[i].to_string() + startend[j].0);
                endquotes.push(quotes[i].to_string() + startend[j].1);
            }
        }

        // Now we simply create and replace all occurences of $beginquote and $endquote
        match tl.find("$endquote") {
            Some(_) => {
                // If endquote exist, then beginquote should also exist, but we don't test it. If
                // it doesn't exist, the code will still work.
                
                // We first get the current length of our return vector, this is important as we
                // will loop over it and insert and the end.
                let ret_len = ret.len();
                for i in 0..ret_len   {
                    // For each if the rets, loop over all quotes
                    let quote_len = beginquotes.len();

                    // Starts at 1 because 0 is handled at the end
                    for j in 1..quote_len  {
                        // Here we use str[i] as a template, and we don't replace that template
                        // until after we have inserted all the new ones
                        let mut ins = str::replace(&ret[i], "$endquote", &endquotes[j]);
                        ins = str::replace(&ins, "$beginquote", &beginquotes[j]);
                        ret.push(ins);
                    }
                    // Replace the original we had in the list with quote[0}
                    ret[i] = str::replace(&ret[i], "$endquote", &endquotes[0]);
                    ret[i] = str::replace(&ret[i], "$beginquote", &beginquotes[0]);
                }
            }
            None => {}
        }
        (ret, replaced)
    }

    pub fn create_attack(&mut self, form: &web::Form) -> (Vec<web::Form>,Vec<String>)    {
        let mut ret = Vec::new();
        let mut replaced = Vec::new();
        let num_params = form.number_of_params();

        // The inner vector holds the strings for each parameter,
        // the outer vector holds for the number of parameters (num_params)
        let mut vec_str_vals: Vec<Vec<String>> = Vec::new();
        let mut vec_str_pars: Vec<Vec<String>> = Vec::new();

        for i in 0..num_params   {
            // Get original / standard values, basically just valid values
            let orig_param = form.get_parameter_index(i);
            let orig_value = form.get_value_index(i);
        
            let val_t = self.val_template.clone();
            let par_t = self.param_template.clone();
            let (str_vals, mut repl_vals) = self.create_strings(val_t, orig_value);
            let (str_pars, mut repl_pars) = self.create_strings(par_t, orig_param);

            vec_str_vals.push(str_vals);
            vec_str_pars.push(str_pars);

            loop    {
                let m = match repl_vals.pop()   {
                    Some(s) => s,
                    None    => break,
                };
                replaced.push(m);
            }
            loop    {
                let m = match repl_pars.pop()   {
                    Some(s) => s,
                    None    => break,
                };
                replaced.push(m);
            }
        }


        // All parameters are injected at once
        if self.inj_in == enums::InjectMethod::All   {
            for val in 0..vec_str_vals[0].len()   {
                for par in 0..vec_str_pars[0].len()   {
                    let mut w = form.clone();
                    for i in 0..num_params   {
                        w.set_parameter_index(i, &vec_str_pars[i][par]);
                        w.set_value_index(i, &vec_str_vals[i][val]);
                    }
                    ret.push(w);
                }
            }
        }
        else if self.inj_in == enums::InjectMethod::Individual   {
            for val in 0..vec_str_vals[0].len()   {
                for par in 0..vec_str_pars[0].len()   {
                    for i in 0..num_params   {
                        let mut w = form.clone();
                        w.set_parameter_index(i, &vec_str_pars[i][par]);
                        w.set_value_index(i, &vec_str_vals[i][val]);
                        ret.push(w);
                    }
                }
            }
            
        }
        else    {
            panic!("Not implemented");
        }
        (ret, replaced)
    }
}

