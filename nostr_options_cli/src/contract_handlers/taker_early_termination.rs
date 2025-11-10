use dcd_manager::manager::init::DcdManager;

pub fn handle() -> crate::error::Result<()> {
    DcdManager::taker_early_termination()?;
    Ok(())
}
