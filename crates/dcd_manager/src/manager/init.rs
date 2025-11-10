pub struct DcdManager;

impl DcdManager {
    pub fn faucet() -> crate::error::Result<()> {
        crate::manager::handlers::faucet::handle()?;
        Ok(())
    }
    pub fn maker_init() -> crate::error::Result<()> {
        crate::manager::handlers::maker_init::handle()?;
        Ok(())
    }
    pub fn maker_funding() -> crate::error::Result<()> {
        crate::manager::handlers::maker_funding::handle()?;
        Ok(())
    }
    pub fn taker_funding() -> crate::error::Result<()> {
        crate::manager::handlers::taker_funding::handle()?;
        Ok(())
    }
    pub fn taker_early_termination() -> crate::error::Result<()> {
        crate::manager::handlers::taker_termination_early::handle()?;
        Ok(())
    }
    pub fn maker_collateral_termination() -> crate::error::Result<()> {
        crate::manager::handlers::maker_termination_collateral::handle()?;
        Ok(())
    }
    pub fn maker_settlement_termination() -> crate::error::Result<()> {
        crate::manager::handlers::maker_termination_settlement::handle()?;
        Ok(())
    }
    pub fn maker_settlement() -> crate::error::Result<()> {
        crate::manager::handlers::maker_settlement::handle()?;
        Ok(())
    }
    pub fn taker_settlement() -> crate::error::Result<()> {
        crate::manager::handlers::taker_settlement::handle()?;
        Ok(())
    }
}
