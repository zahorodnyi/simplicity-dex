use dcd_manager::manager::init::DcdManager;

pub fn handle() -> crate::error::Result<()> {
    DcdManager::faucet()?;
    Ok(())
}
