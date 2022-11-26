# SkyHook-Native

Dynamic library of SkyHook for use with other languages.

## Usage

The following examples are all in C#.

### Functions

The extern functions that you should run are simple.

There are just two; [`start_hook`](#start_hook) and [`stop_hook`](#stop_hook).

### `start_hook`

```cs
// Here, we assign a callback along with starting a hook.
// The NativeEvent type is declared below "Types" header.
public static delegate void HookCallback(NativeEvent ev);

// Here, we have an extern method to invoke.
[DllImport("skyhook", EntryPoint = "start_hook")]
public static extern void StartHook(HookCallback callback);
```

### `stop_hook`

```cs
// No additional information required to provide when stopping the hook.
[DllImport("skyhook", EntryPoint = "stop_hook")]
public static extern void StopHook();
```

### Types

### `NativeEventType`

```cs
public enum NativeEventType {
    KeyPressed,
    KeyReleased
}
```

### `NativeEvent`

```cs
[StructLayout(LayoutKind.Sequential)]
public struct NativeEvent
{
    public readonly ulong Time; // This is the key state update time.
    public readonly NativeEventType Type; // Whether the key is up or down.
    public readonly ushort Label; // Unified label for keys, such as ESC or F11.
    public readonly ushort Key; // Actual key code from native level.
}
```

## Development

```sh
git clone https://git.pikokr.dev/SkyHook/SkyHook-Native #Clone
```

To build, just run [`build.sh`](build.sh) on Linux or MacOS, [`build.ps1`](build.ps1) on Windows.
