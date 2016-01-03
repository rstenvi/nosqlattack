
pub mod enums;
mod header;
mod parameter;

use super::super::helpers;

use hyper::client::Client;
use hyper::client::response;
use hyper::header::Headers;
use hyper::client::RedirectPolicy;


#[derive(Debug, Clone)]
pub struct Form  {
    ip_addr: String,
    port: u16,
    uri: String,
    params: Vec<parameter::Parameter>,
    request: enums::RequestMethod,
    encoding: enums::ParameterEncoding,
    headers: Vec<header::WebHeader>,
    protocol: enums::Protocol,
}

impl Form    {
    /// Create a web form object from strings.
    ///
    /// This function mimics how we get the data from command line, but might need some
    /// pre-processing.
    ///
    /// * url - Of the format http://example.pcom:80
    /// * data - Parameters of the format key=value&key2=value2
    /// * request - Specify request, has the format GET/POST
    /// * encoding - Specify request, has the format URL/JSON
    /// * headers - A vector of raw headers of the format: Type: value
    pub fn new_from_strings(url: &str, data: &str, request: &str, encoding: &str, headers: Vec<String>) -> Form   {

        let (protocol, address, port, path) = Form::split_url(url);
        let req = match request {
            "GET"  => enums::RequestMethod::GET,
            "POST" => enums::RequestMethod::POST,
            _      => enums::RequestMethod::GET,
        };
        let enc = match encoding    {
            "URL"  => enums::ParameterEncoding::URL,
            "JSON" => enums::ParameterEncoding::JSON,
            _      => enums::ParameterEncoding::URL,
        };
        
        let mut head = Vec::new();
        for h in headers.iter() {
            head.push( header::WebHeader::create_string(h) );
        }

        let params = Form::create_parameters(data);
        
        Form    {
            ip_addr: address,
            port: port,
            uri: path,
            params: params,
            request: req,
            encoding: enc,
            headers: head,
            protocol: protocol,
        }

    }

    /// Split the URL info 4 parts (protocol, address, port and path)
    pub fn split_url(url: &str) -> (enums::Protocol, String, u16, String)   {
        let (proto, rest) = helpers::split_in_two(url, "://");
        let (addr, rest) = helpers::split_in_two(&rest, ":");
        let (port, path) = helpers::split_in_two(&rest, "/");

        let p = helpers::parse_panic::<u16>(&port);

        let protocol = match proto.as_ref()  {
            "HTTP" => enums::Protocol::HTTP,
            "HTTPS" => enums::Protocol::HTTPS,
            _       => enums::Protocol::HTTP,
        };

        (protocol, addr, p, path)
    }

    pub fn create_parameters(p: &str) -> Vec<parameter::Parameter>  {
        let params: String = p.to_string();
        let mut param_vec: Vec<parameter::Parameter> = Vec::new();
        let pars: Vec<&str> = params.split("&").collect();
        for p in pars    {
            let pars2: Vec<&str> = p.split("=").collect();
            assert_eq!(pars2.len(), 2);
            param_vec.push( parameter::Parameter::new(pars2[0], enums::ParamType::Text, pars2[1]) );
        }
        param_vec
    }

    pub fn get_parameter_index(&self, ind: usize) -> String {
        self.params[ind].get_parameter()
    }
    pub fn get_value_index(&self, ind: usize) -> String {
        self.params[ind].get_value()
    }
    pub fn set_parameter_index(&mut self, ind: usize, v: &str) {
        self.params[ind].set_parameter(v)
    }
    pub fn set_value_index(&mut self, ind: usize, v: &str) {
        self.params[ind].set_value(v)
    }
    pub fn number_of_params(&self) -> usize {
        self.params.len()
    }
    pub fn get_encoding(&self) -> enums::ParameterEncoding {
        self.encoding.clone()
    }

    pub fn build_url(&self) -> String {
        let prot = match self.protocol  {
            enums::Protocol::HTTP  => "http",
            enums::Protocol::HTTPS => "https",
        };
        format!("{}://{}:{}/{}", prot, self.ip_addr, self.port, self.uri)
    }

    // Get a string that can be sent to curl
    // TODO: Fill in all the headers and extra information
    pub fn get_curl(&self) -> String    {
        let url = self.build_url();
        
        let x: (String, String) = match self.encoding    {
            enums::ParameterEncoding::URL => {
                ("Content-Type: application/x-www-form-urlencoded".to_string(), self.get_parameters_url() )
            }
            enums::ParameterEncoding::JSON => {
                ("Content-Type: application/json".to_string(), self.get_parameters_json() )
            }
            _ => {
                warn!("Encoding not specified");
                ( "".to_string(), "".to_string() )
            }
        };


        match self.request  {
            enums::RequestMethod::POST => {
                str::replace(&format!("curl -i --data-urlencode \"{}\" {}", x.1, url), "$", "\\$")
            }
            enums::RequestMethod::GET  => {
                str::replace(&format!("curl -i -G --data-urlencode \"{}\" \"{}\"", x.1, url), "$", "\\$")
            }
        }
        
    }
    pub fn get_parameters_url(&self) -> String  {
        let mut ret: String = "".to_string();
        for (i, param) in self.params.iter().enumerate() {
            ret = ret + &*param.get_parameter_url();
            if i < self.params.len()-1 {
                ret = ret + "&";
            }
        }
        ret
    }
    pub fn get_parameters_json(&self) -> String  {
        let mut ret = "{".to_string();
        for (i, param) in self.params.iter().enumerate() {
            ret = ret + &*param.get_parameter_json();
            if i < self.params.len()-1 {
                ret = ret + ",";
            }
        }
        ret = ret + "}";
        ret
    }

    pub fn send_request(&self) -> response::Response   {
        let mut client = Client::new();
        client.set_redirect_policy(RedirectPolicy::FollowNone);
        
        // Headers for POST request
        let mut head = Headers::new();
        
        let params = match self.encoding    {
            enums::ParameterEncoding::URL => {
                head.set_raw("Content-Type", vec![b"application/x-www-form-urlencoded".to_vec()]);
                self.get_parameters_url()
            }
            enums::ParameterEncoding::JSON => {
                head.set_raw("Content-Type", vec![b"application/json".to_vec()]);
                self.get_parameters_json()
            }
            _ => {
                panic!("No such encoding");
            }
        };
        for ref h in self.headers.iter()   {
            head.set_raw(h.get_name(), vec![h.get_value().as_bytes().to_vec()]);
        }
        let mut url = self.build_url();

        let res = match self.request  {
            enums::RequestMethod::POST => {
                client.post(&url).body(&params).headers(head).send().unwrap()
            }
            enums::RequestMethod::GET  => {
                url = format!("{}?{}", url, params);
                client.get(&url).send().unwrap()
            }
        };
        res
    }
}


