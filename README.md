# Idea
wait on this signal

(captured with dbus-monitor)
```
signal time=1727643834.629714 sender=:1.12 -> destination=(null destination) serial=82 path=/org/gtk/vfs/mounttracker; interface=org.gtk.vfs.MountTracker; member=Mounted
   struct {
      string ":1.102"
      object path "/org/gtk/vfs/mount/1"
      string "mtp"
      string "mtp:host=SAMSUNG_SAMSUNG_Android_RFCRA1CG6KT"
      string ""
      string ". GThemedIcon multimedia-player multimedia multimedia-player-symbolic multimedia-symbolic"
      string ""
      string ""
      boolean true
      array of bytes "/run/user/1000/gvfs/mtp:host=SAMSUNG_SAMSUNG_Android_RFCRA1CG6KT" + \0
      struct {
         array of bytes "/" + \0
         array [
            dict entry(
               string "host"
               variant                   array of bytes "SAMSUNG_SAMSUNG_Android_RFCRA1CG6KT" + \0
            )
            dict entry(
               string "type"
               variant                   array of bytes "mtp" + \0
            )
         ]
      }
      array of bytes "" + \0
   }
```

Then look if `/run/user/1000/gvfs/mtp:host=SAMSUNG_SAMSUNG_Android_RFCRA1CG6KT` is empty or not:
 * if it is, show a dialog and ask the user to press "Allow" on the phone pop-up. The signal should be re-emitted, but the folder shoud now contain the 'Internal Storage' folder
 * if it contain a folder name 'Internal Storage', open it with the default file explorer
