# DWM Status Bar

This program provides status text for dwm's builtin bar on Linux. It shows battery status, battery capacity, current WIFI connection, and the time in a nice format.

The goal in creating this program was to create something that didn't need configuration, so the program tries to intelligently determine if there is a battery or WiFI, and it won't fail if it there isn't. However, battery and WIFI files can be overridden at the top of the source file.

# How to use

In `.xsession`, `.xinitrc`, or equivilent:

```
while xsetroot -name "`snafu`"
do
    sleep 1
done &
exec dwm
```
