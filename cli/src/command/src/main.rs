use ekc::CreateExpoKit;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(CreateExpoKit::load()?)
}
