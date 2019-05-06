#[macro_use] 
extern crate nickel;
extern crate postgres;
//postgres db connection below here

//Nickel Declaration will need HttpRouter etc..
use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();
    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });
    server.listen("127.0.0.1:6767");
}
