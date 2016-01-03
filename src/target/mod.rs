
pub mod databases;  // Used by main
mod web;
mod injection;

use std::vec::Vec;
use std::boxed::Box;

pub struct WebTarget    {
    webform: web::Form,
    inject: injection::Inject,
}

impl WebTarget  {
    pub fn new(url: &str, data: &str, request: &str, encoding: &str, headers: Vec<String>, inj_path: &str) -> WebTarget   {
        WebTarget   {
            webform: web::Form::new_from_strings(url, data, request, encoding, headers),
            inject: injection::Inject::new(inj_path),
        }

    }

    pub fn attack(&mut self) -> Result<(u32,u32,u32),()>    {
        // We create a copy of our web form before we use
        let mut form = self.webform.clone();

        let baseline = self.inject.run_inject_test(&mut form);

        let res = self.inject.inject_test_all_form(&mut form, &baseline);


        let mut confident = 0;
        let mut possible = 0;
        let mut uncertain = 0;

        for r in res.iter()  {
            if *r >= 10  { confident += 1; }
            else if *r >= 5  { possible  += 1; }
            else if *r >= 1  { uncertain += 1; }
        }
        Ok( (confident, possible, uncertain) )
            
    }
}



pub struct AuthTarget   {
    db: Box<databases::DBAttack>,
    dbs: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum TargetResult   {
    UnableToConnect,
    UnableToExecuteCmd,

}


impl AuthTarget {
    pub fn new(db: Box<databases::DBAttack>) -> AuthTarget  {
        AuthTarget  {
            db: db,
            dbs: Vec::new(),
        }
    }

    pub fn attack(&mut self) -> Result<(),TargetResult>  {
        let dbs = match self.db.list_databases()    {
            Ok(s)  => s,
            Err(_) => return Err(TargetResult::UnableToConnect),
        };
        self.dbs = dbs;

        if self.dbs.len() > 0    {
            println!("Found the following databases: {:?}", self.dbs);
        }
        else    {
            println!("Authentication attack seeminly successful, but no records retrieved");
        }
        Ok(())
    }
}


