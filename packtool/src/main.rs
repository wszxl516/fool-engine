use packtool::GamePackage;
fn main() -> anyhow::Result<()> {
    let mut gp = GamePackage::default();
    gp.add_folder("./assets")?;
    gp.write_to_file("./a.pak", true)?;
    let gp = GamePackage::read_from_file("./a.pak")?;
    match gp.extract("engine/utils.lua") {
        Some(data) => {
            println!("{}", String::from_utf8_lossy(data))
        }
        None => {}
    }
    println!("{}", gp);
    Ok(())
}
