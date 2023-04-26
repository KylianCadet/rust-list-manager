use crate::cli::CliHandlerTrait;

pub struct ListManagerHandler {}

impl CliHandlerTrait for ListManagerHandler {
    fn handle(
        &self,
        line: String,
    ) -> Result<Option<std::process::ExitCode>, Box<dyn std::error::Error>> {
        println!("in handler {}", line);
        Ok(None)
    }
}
