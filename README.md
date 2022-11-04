# GitHub AuthorizedKeysCommand (hubakc)

Heavily inspired by https://github.com/sequencer/gitakc . It allows someone login to the server using their **ssh keys on GitHub**.

1. Put the config.toml in `/etc/hubakc/config.toml`, and add user map.
2. Put the binary in `/usr/local/bin/hubakc`. Make sure that the owner is root and the permission is 755.
3. Edit sshd config as below:

```
AuthorizedKeysCommand /usr/local/bin/hubakc
AuthorizedKeysCommandUser nobody
```

## Example

Given the config file `config.toml`

```toml
ttl = 3600
timeout = 15
cache_folder = "/tmp/hubakc"
[user_map]
mgt = "Enter-tainer"
```

```bssh
> hubakc mgt
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJgIynaQvTeYZ5iPigLnYRkRThxE04U7ACjuHRkQBAk+
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIL2C/osNvVrilQDE3T/tTK9TRQ0+xVSbFU6wN5oIr2Fv
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINKvQJ1fgkAS2yuy+cbl8iYaiw0IR4lkQIJIKgj7liax
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIKSkhuW2F4UCnIa4b2m0gUt4A5Bv+UYGsUYEl9VmLJbu
```
