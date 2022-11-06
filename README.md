# SkyHook-Native

dynamic library of SkyHook for use with other languages

## Usage

The following examples are all in C#

### Functions

The exported functions are simple. Just `start_hook` and `stop_hook`

### start_hook

```cs
public static delegate void HookCallback(NativeEvent event);

[DllImport("skyhook", EntryPoint = "start_hook")]
public static extern void StartHook(HookCallback callback);
```

### stop_hook

```cs
[DllImport("skyhook", EntryPoint = "stop_hook")]
public static extern void StopHook();
```

### Types

### NativeEventType

```cs
public enum NativeEventType {
  KeyPressed,
  KeyReleased
}
```

### NativeEvent

```cs
[StructLayout(LayoutKind.Sequential)]
public struct NativeEvent
{
  public readonly ulong Time;
  public readonly EventType Type;
  public readonly uint Key;
}
```

## Development

```sh
git clone https://git.pikokr.dev/SkyHook/SkyHook-Native #Clone
```

to build, just run `build.sh` on Linux of MacOS, `build.ps1` on Windows
