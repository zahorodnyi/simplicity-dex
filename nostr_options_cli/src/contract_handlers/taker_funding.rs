use dcd_manager::manager::init::DcdManager;

pub fn handle() -> crate::error::Result<()> {
    DcdManager::taker_funding()?;

    Ok(())
}
