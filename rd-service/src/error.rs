pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[derive(facet::Facet)]
#[facet(derive(Error))]
#[repr(C)]
pub enum Error {
    // Service is already running
    ServiceAlreadyRunning,
}
