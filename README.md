## NetNS Exec

This is super simple command for executing a process within a Linux network namespace. I personally use it to run my whole desktop within a namespace where there's only a [wireguard](https://www.wireguard.com/).

The wireguard dev himself suggests creating all unencrypted network interfaces (like `eth0` or `wlan0`) together with the wireguard interface within a certain network namespace and then you move the wireguard interface out of there while leaving its socket in the original namespace (together with the unencrypted ones). That way, your `init` (eg. main) network namespace will only have a wireguard interface so everything goes over that interface (and no fiddling with routes needed). This is obviously really cool and what you'd probably want to do if you can... unfortunately it can be a bit difficult to make all that work, starting dhcpd wpa_supplicant - not to mention iwd in a different namespace. So, this instead enables me to run my desktop within a namespace into which I've moved the wireguard interface (leaving the wlan0 etc. in the `init` namespace). So - the next best solution.

You must create the network namespace before you can run this command, when you've created it - you can run this like so:

```sh
netns-exec <namespace> cmdline here
```

A more concrete example would be:
```sh
netns-exec private sway
```