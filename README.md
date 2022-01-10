# scribec-rs

a [scribe](https://github.com/facebookarchive/scribe) client for rust

[![crates.io](https://img.shields.io/crates/v/scribec.svg)](https://crates.io/crates/scribec)
![License](https://img.shields.io/crates/l/scribec.svg)


usage
---

```
scribec admin version
scribec -H remote_host -p 1463 admin counters
echo "abc" | scribec cat test
tail -f /var/log/nginx/access.log | scribec std_logger access_log
```

todos
---
- [ ] reconnect on failure
- [ ] std logger read bulk
