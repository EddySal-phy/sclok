# sclok

![Screenshot](https://github.com/EddySal-phy/sclok/blob/master/images/sclok_2.png)

sclok is a lightweight, minimal, floating clock overlay for Linux that can stay visible over your browser or full-screen applications.

Written in Rust using the `eframe` ([egui](https://github.com/emilk/egui)) GUI framework.

## Usage

```console

unix@desk:~$ usage : sclok
    -S    Display seconds
    -c    Set text/number color
    -b    Set background color
    -t    Set background transparency [0-255] (default: 160)
    -lc   List colours
    -h    Help
```
Right click the frame panel to change the font.


## Install

Available for **Linux** (.deb).


### Debian (Linux)

```console

#clone repository
unix@desk:~$ git clone https://github.com/EddySal-phy/sclok.git
unix@desk:~$ cd sclok
unix@desk:~$ sudo dpkg -i install/debian/sclok_2.0.0-1_amd64.deb

#run
unix@desk:~$ sclok
```
