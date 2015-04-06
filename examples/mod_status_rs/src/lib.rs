extern crate libc;

#[macro_use]
extern crate apache2;

use apache2::{Request, Status, get_server_description, get_server_built, show_mpm};

apache2_module!(status_rs_module, status_rs_handler, c_status_rs_handler, b"mod_status_rs\0");


fn status_rs_handler(r: &Request) -> Status {
   if r.handler().unwrap() != "server-status-rs" {
      return Status::DECLINED
   }

   let conn = r.connection().unwrap();

   r.set_content_type("text/html");

   r.write("<!doctype html><html><head><meta charset=\"utf-8\"><title>Apache Status</title></head><body>");

   let server_name = r.escape_html(r.server_name().unwrap()).unwrap();
   let local_ip = conn.local_ip().unwrap();
   r.write(format!("<h1>Apache Server Status for {} (via {})</h1>", server_name, local_ip));

   let server_description = get_server_description().unwrap();
   r.write(format!("<p>Server Version: {}</p>", server_description));

   let mmp = show_mpm().unwrap();
   r.write(format!("<p>Server MPM: {}</p>", mmp));

   let server_built = get_server_built().unwrap();
   r.write(format!("<p>Server Built: {}</p>", server_built));

   let client_ip = conn.client_ip().unwrap();
   r.write(format!("<p>Client IP: {}</p>", client_ip));

   r.write("</body></html>");

   Status::OK
}
