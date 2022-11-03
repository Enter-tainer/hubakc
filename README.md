# GitHub AuthorizedKeysCommand (hubakc)

heavily inspired by https://github.com/sequencer/gitakc

1. Put the config.yml in `/etc/hubakc/config.toml`
2. Put the binary in `/usr/local/bin/hubakc`
3. edit sshd config and set `AuthorizedKeysCommand /usr/local/bin/hubakc`