
extern crate getopts;
extern crate mongodb;   // https://github.com/mongodb-labs/mongo-rust-driver-prototype
extern crate hyper;     // https://github.com/hyperium/hyper
extern crate rustc_serialize;
//extern crate xml;   // https://github.com/netvl/xml-rs
extern crate rand;
extern crate time;
extern crate ini;
#[macro_use] extern crate log;

pub mod logger;
use log::LogLevelFilter;

pub mod target;
pub mod error;
pub mod helpers;

use getopts::Options;
use std::env;

use std::boxed::Box;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
    println!("Some examples");
    println!("\t{} -a web -u http://127.0.0.1:3000/webapp -d param1=data -X POST -e JSON -i inject.ini", program);
    println!("\t{} -a auth -u 127.0.0.1 -b mongodb -p 27017", program);
}


fn main() {
    // Find my own program name
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("v", "verbose", "Increase verbosity");
    opts.optopt("a", "attack", "Attacks to perform", "(auth | web)");
    opts.optopt("u", "url", "Web Application URL", "http://example.com:3000/webapp");
    opts.optopt("d", "data", "Parameters to send (as URL encoding, even if JSON encoding",
                "param1=data&param2=otherdata");
    opts.optopt("X", "request", "Request method", "GET/POST");
    opts.optopt("e", "encoding", "Encoding", "URL/JSON");
    opts.optopt("i", "inject", ".ini file containing the injection attacks", "inject.ini");

    opts.optmulti("H", "Header", "Header key and value", "\"Cookie: sessionid=deadbeef\"");
    opts.optopt("b", "db", "Database that is in use",  "(mongodb | couchdb | unknown)");
    opts.optopt("p", "port", "Database port", "NUMBER");


    // Start parsing the options
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            error!("Unable to parse options: {}", f);
            print_usage(&program, opts);
            return;
        }
    };

    // If help is specified, we show help menu and exit
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Check verbose and initialize logger based on it
    if matches.opt_present("v") {
        let _ = logger::init(LogLevelFilter::Info);
    }
    else    {
        let _ = logger::init(LogLevelFilter::Warn);
    }


    let attack_mode = match matches.opt_str("a")    {
        Some(s) => s,
        None    => {
            error!("Attack mode must be provided");
            print_usage(&program, opts);
            return;
        }
    };


    let authattack = if attack_mode == "auth" || attack_mode == "all"   {
        let port = match matches.opt_str("p") {
            Some(s) => s.parse::<u16>().unwrap(),
            None    => {
                error!("Database port must be given in auth attacks");
                print_usage(&program, opts);
                return;
            }
        };
        let url = match matches.opt_str("u") {
            Some(s) => s,
            None    => {
                error!("URL must be specified for web attacks");
                print_usage(&program, opts);
                return;
            }
        };
        
        let db: Box<target::databases::DBAttack> = match matches.opt_str("b") {
            Some(s) => {
                match s.as_ref() {
                    "mongodb" => Box::new(target::databases::mongodb::MongoDB::new(&url, port))
                        as Box<target::databases::DBAttack>,
                    "couchdb" => Box::new(target::databases::couchdb::CouchDB::new(&url, port))
                        as Box<target::databases::DBAttack>,
                    _         => {
                        error!("DB {} is not supported", s);
                        print_usage(&program, opts);
                        return;
                    }
                }
            }
            None => {
                error!("DB must be specified in auth attack");
                print_usage(&program, opts);
                return;
            }
        };
        Some(target::AuthTarget::new(db))
        
    }
    else    {
        None
    };

    let webattack: Option<target::WebTarget> = if attack_mode == "web" || attack_mode == "all"   {
        let url = match matches.opt_str("u") {
            Some(s) => s,
            None    => {
                error!("URL must be specified for web attacks");
                print_usage(&program, opts);
                return;
            }
        };
        let request = match matches.opt_str("X") {
            Some(s) => s,
            None    => "GET".to_string(),
        };
        let encoding = match matches.opt_str("e") {
            Some(s) => s,
            None    => "URL".to_string(),
        };
        let headers: Vec<String> = matches.opt_strs("H");

        let ini_file = match matches.opt_str("i") {
            Some(s) => s,
            None    => "data/inject.ini".to_string(),
        };
        let data = match matches.opt_str("d") {
            Some(s) => s,
            None    => {
                error!("Parameters must be specified for web attacks");
                print_usage(&program, opts);
                return;
            }
        };


        Some(
            target::WebTarget::new(
                &url,
                &data,
                &request,
                &encoding,
                headers,
                &ini_file,
            )
        )
    }
    else    {
        None
    };

    match attack_mode.as_ref()  {
        "all" =>    {
            let _ = authattack.unwrap().attack();
            let _ = webattack.unwrap().attack();
        }
        "web" =>    {
            let _ = webattack.unwrap().attack();
        }
        "auth" =>   {
            let _ = authattack.unwrap().attack();
        }
        _      => {}
    }
    
}








//------------------- ALL tests -------------------------------


#[cfg(test)]
mod tests {
    use super::*;   // Import all in main module
    use std::env;


    //------------ Test helper modules ------------------------


    // format: ip.addr;webport;dbport:
    fn get_variables(env_key: &str) -> String  {
        let val = match env::var(env_key) {
            Ok(o) => o,
            Err(e) => panic!("couldn't interpret {}: {}", env_key, e),
        };
        return val.to_string();
    }

    fn target_authtarget(db: Box<target::databases::DBAttack>)   {
        // Get the name of DB name
        let name: String = {
            let tmp = db.get_name();
            tmp.to_string()
        };

        let mut attack = target::AuthTarget::new(db);

        let ret = attack.attack();
        assert!(ret == Ok(()), "Attack on DB {} did not return Ok, it returned {:?}", name, ret);
    }



    //---------------- Tests that are executed ---------------------------

    #[test]
    fn target_databases()   {
        let ip = get_variables("NOSQLTEST");
        let mongo = Box::new(target::databases::mongodb::MongoDB::new(&ip, 27017)) as 
            Box<target::databases::DBAttack>;
        let couch = Box::new(target::databases::couchdb::CouchDB::new(&ip, 5984)) as 
            Box<target::databases::DBAttack>;

        target_authtarget(mongo);
        target_authtarget(couch);
    }

    #[test]
    fn target_web() {
        let ip = get_variables("NOSQLTEST");
        let url_base = format!("http://{}:3000/", ip);
        let params = [
            (format!("{}searchuser", url_base), "search=name", "GET", "URL", Vec::new(), "data/inject.ini"),
            (format!("{}age", url_base), "ager=1990", "POST", "URL", Vec::new(), "data/inject.ini")
        ];


        let result = [
            2,
            18
        ];

        for i in 0..params.len()    {
            let mut attack = target::WebTarget::new(
                &params[i].0,
                params[i].1.clone(),
                params[i].2.clone(),
                params[i].3.clone(),
                params[i].4.clone(),
                params[i].5.clone()
            );
            let ret = attack.attack();
            let (conf, poss, uncer) = match ret   {
                Ok(o) => o,
                Err(_) => panic!("Attack #{} on web form did not return Ok", i),
            };
            println!("{} {} {}", conf, poss, uncer);

            // The server doesn't always behave the same way, do difficult to make
            // an exact comparison.
            assert_eq!( (conf + poss + uncer), result[i]);
        }
    }
}

