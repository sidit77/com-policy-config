use com_policy_config::{IPolicyConfig, PolicyConfigClient};
use windows::core::{Result, PCWSTR};
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::{
    eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED, STGM_READ,
};

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        println!("All Devices:");
        let device_collection = enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
        for i in 0..device_collection.GetCount()? {
            let device = device_collection.Item(i)?;
            let property_store = device.OpenPropertyStore(STGM_READ)?;
            let name = property_store.GetValue(&PKEY_Device_FriendlyName)?;
            println!("  {}", name.Anonymous.Anonymous.Anonymous.pwszVal.display());
        }

        let selected_device = device_collection.Item(0)?;
        let property_store = selected_device.OpenPropertyStore(STGM_READ)?;
        let name = property_store.GetValue(&PKEY_Device_FriendlyName)?;
        println!(
            "Selected Device: {}",
            name.Anonymous.Anonymous.Anonymous.pwszVal.display()
        );

        let device_id = PCWSTR(selected_device.GetId()?.0);
        let policy_config: IPolicyConfig = CoCreateInstance(&PolicyConfigClient, None, CLSCTX_ALL)?;
        policy_config.SetDefaultEndpoint(device_id, eConsole)?;
    }

    unsafe {
        CoUninitialize();
    }
    Ok(())
}
