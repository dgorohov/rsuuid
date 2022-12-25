Very simple tool to generate UUID v1 or v4.

The usage is pretty simple:

```
# rsuuid v1
ddb60adc-2b70-11ec-802a-010203040506

# rsuuid v4
a6b0a1f5-fcd3-4a83-bcf9-30334aa8cc41
```

It prints nothing more since I'd like to put the content directly into the clipboard, like this:
```
# rsuuid v1 | pbcopy
```

Installing:

On Linux:
```
$ curl https://github.com/dgorohov/rsuuid/releases/download/v0.1.3/rsuuid-linux-amd64 -O /usr/local/bin/rsuuid
$ chmod +x /usr/local/bin/rsuuid
```

On Mac:

```
$ curl https://github.com/dgorohov/rsuuid/releases/download/v0.1.3/rsuuid-macos-amd64 -O /usr/local/bin/rsuuid
$ chmod +x /usr/local/bin/rsuuid
```

Brew support will come later, once I'd have a time.
