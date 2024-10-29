use std::io::Error;
use ls::App;

fn main() -> Result<(), Error> {
    let app: App = App::build();
    app.run()?;
    Ok(())
}
