//This is a Small Sample of Ureq Functionality

///Ureq’s first priority is being easy for you to use. It’s great for anyone who wants a low-overhead HTTP client that just gets the job done. Works very well with HTTP APIs. Its features include cookies, JSON, HTTP proxies, HTTPS, and charset decoding.

///Ureq is in pure Rust for safety and ease of understanding. It avoids using unsafe directly. It uses blocking I/O instead of async I/O, because that keeps the API simple and keeps dependencies to a minimum. For TLS, ureq uses rustls.
//First step HTTP Get Simple Request Then how to add path parameters, and finally how to add HTTP parametes to it

fn main() {
    //http_sample_1()
    http_sample_2()
}

fn http_sample_2() {
    //This includes a path parameter interpolation
    let todo_id = 1;
    let get_url_path_interpolation =
        format!("https://jsonplaceholder.typicode.com/todos/{}", todo_id);
    let res: ureq::Response = ureq::get(get_url_path_interpolation.as_str())
        .call()
        .unwrap();
    println!(
        "HTTP Get, with path interpolation, no query parameters: \n- URL: {}\n- res code: {},\n- res body:\n{}",
        get_url_path_interpolation,
        res.status(),
        res.into_string().unwrap(),
    );
}

#[allow(dead_code)]
fn http_sample_1() {
    let url = "https://jsonplaceholder.typicode.com/posts";
    let res: ureq::Response = ureq::get(url).call().unwrap();
    println!(
        "HTTP Get, no path interpolation, no query parameters: \n- URL: {}\n- res code: {},\n- res body:\n{}",
        url,
        res.status(),
        res.into_string().unwrap(),
    );
}
