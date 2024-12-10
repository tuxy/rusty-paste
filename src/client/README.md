# rusty-paste client

Very simple client, which can be interchanged with a slightly more complicated curl command.

## Usage
```Usage: client.exe [OPTIONS] <URL>

Arguments:
  <URL>

Options:
  -c, --content <CONTENT>
  -p, --post
  -h, --help               Print help
  -V, --version            Print version
```

To post a paste:
`./client -p --u http://{url} -c "This is a new paste"`

To get a paste:
`./client -u http://{url}/whateverpastehere`