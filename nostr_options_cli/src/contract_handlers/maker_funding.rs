use dcd_manager::manager::init::DcdManager;

pub fn handle() -> crate::error::Result<()> {
    DcdManager::maker_funding()?;

    Ok(())
}
