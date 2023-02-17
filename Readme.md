# com-policy-config

COM definitions for the undocumented `IPolicyConfig` interface. This interface can be used to change to the default audio device on windows. 

This is a manual port of [PolicyConfig.h](https://github.com/Belphemur/AudioEndPointLibrary/blob/master/DefSound/PolicyConfig.h).

More additional resources:

[Link 1]([https://github.com/KudoAmine/Toggle-Loudness-Equalization/blob/24445e5697d38d7d082412c199673662d2b58620/Toggle%20Loudness%20Equalization/Form1.vb#L174-L175)

[Link 2](https://learn.microsoft.com/en-us/answers/questions/669471/how-to-control-enable-audio-enhancements-with-code?orderby=helpful)

**Important**: It is possible I made a mistake during the port or messed up some type. Take everything with a grain of salt.

## Example

```rust
use windows::core::{PCWSTR, Result};
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::{DEVICE_STATE_ACTIVE, eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance, COINIT_MULTITHREADED, CoInitializeEx, CoUninitialize, STGM_READ};
use com_policy_config::{IPolicyConfig, PolicyConfigClient};

fn main() -> Result<()>{

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

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
        println!("Selected Device: {}", name.Anonymous.Anonymous.Anonymous.pwszVal.display());

        let device_id = PCWSTR(selected_device.GetId()?.0);
        let policy_config: IPolicyConfig = CoCreateInstance(&PolicyConfigClient, None, CLSCTX_ALL)?;
        policy_config.SetDefaultEndpoint(&device_id, eConsole)?;

        CoUninitialize();
    }

    Ok(())
}
```


## License
MIT License
