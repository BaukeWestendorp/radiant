use crate::project;

#[derive(Default)]
pub struct SacnOutputService {}

impl SacnOutputService {
    pub fn new(_config: &project::SacnOutputConfig) -> Self {
        Self {}
    }
}

impl rd_service::Delegate for SacnOutputService {
    type Error = anyhow::Error;
    type Data = ();

    fn on_start(&self) -> Result<(), Self::Error> {
        log::error!("FIXME: Implement SacnOutputService `on_start`");
        Ok(())
    }

    fn on_frame(&self, _data: Self::Data) -> Result<(), Self::Error> {
        log::debug!("FIXME: Implement SacnOutputService `on_frame`");
        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        log::error!("FIXME: Implement SacnOutputService `on_stop`");
        Ok(())
    }
}
