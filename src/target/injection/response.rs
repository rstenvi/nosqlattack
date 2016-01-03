
use super::super::super::helpers;

use hyper::status::StatusCode;
use hyper::header::Headers;

use time::Duration;

pub struct InjResponse  {
    // Can do some math on duration https://doc.rust-lang.org/time/time/struct.Duration.html
    time: Duration,     // How long it took to complete the request
    code: StatusCode,   // Return code, 200 OK etc
    pub output: String,     // The actual response
    headers: Headers,   // All the headers
}

impl InjResponse    {
    pub fn new(time: Duration, code: StatusCode, output: &str, headers: &Headers) -> InjResponse    {
        InjResponse {
            time: time,
            code: code,
            output: output.to_string(),
            headers: headers.clone(),
        }
    }
    pub fn code_ok(&self) -> bool { self.code == StatusCode::Ok }
    pub fn code_equal(&self, i: &InjResponse) -> bool { self.code == i.code }
    pub fn get_ms_difference(&self, i: &InjResponse) -> i64 {
        i.time.num_milliseconds() - self.time.num_milliseconds()
    }
    pub fn cmp_output_len(&self, i: &InjResponse) -> i32  {
        i.output.len() as i32 - self.output.len() as i32
    }

    pub fn cmp(&self, other: &InjResponse, echo: Vec<String>) -> i32   {
        // 0 indicate no difference
        let mut ret = 0;

        // Here we try to quantify the certainty that the form is injectable.
        // The most certain attacks are echo of random input and significant
        // time delay
        // A different response code is fairly good sign that something is injectable.
        // The weakest is different output length.

        if self.get_ms_difference(other) > 9000                 { ret += 10; }
        if helpers::search_vector(echo, other.output.clone()) == true    { ret += 10; }
        if self.code_ok() && self.code_equal(other) == false    { ret += 5;  }

        // Different in length is not a very effective way to measure, so there is little certainty
        if self.cmp_output_len(other).abs() > 10                { ret += 1;  }

        ret
    }
}

