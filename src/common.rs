use afire::{internal::common::remove_address_port, Request};

pub fn get_ip(req: &Request) -> String {
    let mut ip = remove_address_port(&req.address);
    if ip == "127.0.0.1" {
        if let Some(i) = req.headers.iter().find(|x| x.name == "X-Forwarded-For") {
            ip = i.value.to_owned();
        }
    }

    ip
}
