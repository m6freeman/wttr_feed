mod constants;
pub use constants::CITY;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let city: &str = constants::CITY;
    let url: String = format!("https://wttr.in/{city}?format=\"%C+%t+%w+%p\"");
    let response = minreq::get(url).send()?;
    let body: &str = response.as_str()?;
    if response.status_code == 200 {
        println!("{}", body.replace("\"", ""));
    };
    Ok(())
}
