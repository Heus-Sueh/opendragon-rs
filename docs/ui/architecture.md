# OpenDragon GUI Architecture

This document describes the architectural patterns, state synchronization model, and design decoupling of the **OpenDragon GUI** (`apps/opendragon-gui`).

---

## 1. High-Level Architecture Overview

```text
┌────────────────────────────────────────────────────────┐
│                   OpenDragon GUI                       │
│  (gpui-ce, Views, Components, Theme, State Model)      │
└─────────────────────────┬──────────────────────────────┘
                          │
                   DeviceBackend (Trait)
                          │
         ┌────────────────┴────────────────┐
         ▼                                 ▼
┌──────────────────┐             ┌──────────────────┐
│ MockDeviceBackend│             │RealDeviceBackend │
│ (Development &   │             │(Physical HID via │
│  Unit Tests)     │             │ opendragon-hid)  │
└──────────────────┘             └─────────┬────────┘
                                           │
                                 ┌─────────┴────────┐
                                 ▼                  ▼
                         opendragon-devices   opendragon-hid
```

---

## 2. Decoupling UI from Protocol

A primary architectural requirement of OpenDragon is that the GUI has **zero awareness of raw HID protocols**:
- No byte-level packet assembly or report ID manipulation.
- No direct calls to low-level endpoints or hidraw handles.
- All hardware interactions are mediated via typed models from `opendragon-core`:
  - `LightingConfig`, `LightingMode`, `RgbColor`
  - `ActuationPoint` (with automatic 0.1mm - 4.0mm physical range clamping)
  - `RapidTriggerConfig` (boolean enabled, press/release sensitivity mm)
  - `PollingRate` (Hz125, Hz250, Hz500, Hz1000)
  - `ProfileId` (Profile 1, 2, 3...)

---

## 3. The `DeviceBackend` Abstraction

The trait `DeviceBackend` in `src/services/devices.rs` serves as the interface between the presentation layer and hardware:

```rust
pub trait DeviceBackend: Send + Sync {
    fn enumerate_devices(&mut self) -> Result<Vec<DeviceSummary>, OpenDragonError>;
    fn get_device_info(&self, device_id: &str) -> Result<DeviceInfo, OpenDragonError>;
    fn get_capabilities(&self, device_id: &str) -> Vec<Capability>;

    fn get_lighting(&self, device_id: &str) -> Result<LightingConfig, OpenDragonError>;
    fn set_lighting(&mut self, device_id: &str, config: &LightingConfig) -> Result<(), OpenDragonError>;

    fn get_actuation(&self, device_id: &str) -> Result<ActuationPoint, OpenDragonError>;
    fn set_actuation(&mut self, device_id: &str, point: ActuationPoint) -> Result<(), OpenDragonError>;

    fn get_rapid_trigger(&self, device_id: &str) -> Result<RapidTriggerConfig, OpenDragonError>;
    fn set_rapid_trigger(&mut self, device_id: &str, config: &RapidTriggerConfig) -> Result<(), OpenDragonError>;

    fn get_polling_rate(&self, device_id: &str) -> Result<PollingRate, OpenDragonError>;
    fn set_polling_rate(&mut self, device_id: &str, rate: PollingRate) -> Result<(), OpenDragonError>;

    fn get_profiles(&self, device_id: &str) -> Result<Vec<ProfileSummary>, OpenDragonError>;
    fn set_profile(&mut self, device_id: &str, profile_id: ProfileId) -> Result<(), OpenDragonError>;
    fn add_profile(&mut self, device_id: &str, name: String) -> Result<ProfileId, OpenDragonError>;
    fn delete_profile(&mut self, device_id: &str, profile_id: ProfileId) -> Result<(), OpenDragonError>;

    fn get_macros(&self, device_id: &str) -> Result<Vec<MacroItem>, OpenDragonError>;
    fn save_macro(&mut self, device_id: &str, item: MacroItem) -> Result<(), OpenDragonError>;
    fn delete_macro(&mut self, device_id: &str, macro_id: &str) -> Result<(), OpenDragonError>;

    fn get_probe_report(&self, device_id: &str) -> Result<String, OpenDragonError>;
}
```

### Real vs. Mock Backends
- **`MockDeviceBackend`**: Emulates the **Redragon Kumara Pro K552RGB-PRO** with realistic defaults, physical actuation validation, rapid trigger states, and multi-profile storage. Enabled via `--features mock-device`.
- **`RealDeviceBackend`**: Scans the system via `opendragon_hid::discovery::DeviceEnumerator`, matches against `opendragon_devices::registry::DeviceRegistry`, and translates high-level domain operations into typed device driver calls.

---

## 4. Reactive State Management with GPUI-CE

- **`AppState` Entity**: Holds connected device lists, active device configuration state, user preferences, and notification toasts.
- **`Entity<AppState>`**: Wrapped in a GPUI entity. Changes trigger fine-grained view re-renders via `cx.notify()`.
- **Optimistic UI with Toast Feedback**: Immediate local state updates give instant feedback to user gestures; hardware operations run synchronously/asynchronously and emit feedback via non-intrusive toast notifications.
