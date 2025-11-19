### Only supports Windows (for now).

### Usage
```
> killtask2 <port1> <port2> <port3> ...
```

Equivalently, you could just run `netstat -ano | findstr :<port>` followed by `taskkill /F /PID <PID>`, but using Rust was cooler.