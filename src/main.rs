extern crate iron;
extern crate router;
extern crate urlencoded;

use router::Router;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");
    router.get("/*", handle_404, "not_found");
    println!("Server on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    res.set_mut(status::Ok);
    res.set_mut("text/html; charset=utf-8".parse::<Mime>().unwrap());
    res.set_mut("<html><head><title>GCD Calculator</title></head><body><h1>GCD Calculator</h1><p>This calculator computes the greatest common divisor of two numbers.</p><form method=\"post\" action=\"/gcd\"><input type=\"text\" name=\"n\"><input type=\"text\" name=\"n\"><button type=\"submit\">Compute GCD</button></form></body></html>");
    Ok(res)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn handle_404(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    res.set_mut(status::NotFound);
    res.set_mut("text/html; charset=utf-8".parse::<Mime>().unwrap());
    res.set_mut("<html><head><title>404 Not Found</title></head><body><h1>404 Not Found</h1><p>The page you requested could not be found.</p><a href=\"/\">Go to the GCD calculator</a></body></html>");
    Ok(res)
}

fn post_gcd(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    res.set_mut(status::Ok);
    let form_data = match req.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            res.set_mut(status::BadRequest);
            res.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(res);
        }
        Ok(map) => map,
    };
    let unparsed_number = match form_data.get("n") {
        None => {
            res.set_mut(status::BadRequest);
            res.set_mut("Error: missing parameter `n1`\n");
            return Ok(res);
        }
        Some(nums) => nums
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_number {
        match u64::from_str(unparsed) {
            Err(_) => {
                res.set_mut(status::BadRequest);
                res.set_mut(format!("Error: `{}` is not a number\n", unparsed));
                return Ok(res);
            }
            Ok(n) => { numbers.push(n); }
        }
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    res.set_mut(status::Ok);
    res.set_mut("text/html; charset=utf-8".parse::<Mime>().unwrap());
    res.set_mut(format!("The greatest common divisor of {:?} is {}\n
    </br><a href=\"/\">Compute another GCD</a>\n", numbers, d));
    Ok(res)
}