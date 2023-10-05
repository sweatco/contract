use proc_macros::make_integration_version;

#[make_integration_version]
pub trait ContractNameInterface {
    fn init() -> Self
    where
        Self: Sized;

    fn init_with_name(name: String) -> Self
    where
        Self: Sized;

    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);
}
