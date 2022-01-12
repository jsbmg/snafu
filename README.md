# DWM Status Bar

This program provides status text for dwm's builtin bar on Linux. It shows battery status, battery capacity, current WIFI connection, and the time in a nice format. It automatically finds the battery and WIFI device, but those can also be overriden in the source file. 

# How to use

In `.xsession`, `.xinitrc`, or equivilent:

```
while xsetroot -name "`snafu`"
do
    sleep 1
done &
exec dwm
```
