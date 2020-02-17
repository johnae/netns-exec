## NetNS Exec

This is a super simple command for executing a process within a Linux network namespace. I personally use it to run my whole desktop within a namespace where there's only a [wireguard](https://www.wireguard.com/) interface, but you could use it for other reasons as well.

The [wireguard](https://www.wireguard.com/) dev himself suggests creating all unencrypted network interfaces (like `eth0` or `wlan0`) together with the wireguard interface within a certain network namespace and then you move the wireguard interface out of there into the `init` (eg. main) network namespace while leaving its socket in the original namespace (together with the unencrypted ones). That way, your `init` network namespace will only have a wireguard interface so everything goes over that interface (and no fiddling with routes etc needed).
This is obviously really cool and what you'd probably want to do if you can... unfortunately it can be a bit difficult to make all that work, starting dhcpd, wpa_supplicant or iwd in a different network namespace. So, this instead enables me to run my desktop within a namespace into which I've moved only the wireguard interface, leaving the wlan0 etc. in the `init` namespace.

You must create the network namespace before you can run this command, when you've created it - you can run this like so:

```sh
netns-exec <namespace> cmdline here
```

A more concrete example would be:
```sh
netns-exec private sway
```

For this to be runnable as a normal user without sudo, you need to set the `setuid` bit (and the executable should be owned by root ofc). As soon as we've switched network namespace (a privileged operation), we drop privileges.