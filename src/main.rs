use windows_sys::service::{ServiceMainFunction, ServiceStatusHandle, ServiceTableEntry, SERVICE_STATUS};
use windows_sys::winapi::{DWORD, SERVICE_ACCEPT_STOP, SERVICE_CONTROL_STOP, SERVICE_RUNNING, SERVICE_START_PENDING, SERVICE_STOPPED, SERVICE_STOP_PENDING};



mod data_collection;
use data_collection::data_collection;
mod data_transfer;
use data_transfer::data_transfer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_transfer = tokio::spawn(data_transfer());

    let data_collection = tokio::spawn(data_collection());


    let _err =tokio::join!(data_transfer, data_collection);

    Ok(())
}