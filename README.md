# scribec-rs

a [scribe](https://github.com/facebookarchive/scribe) client for rust

usage
---

```
scribec admin version
scribec -H remote_host -p 1463 admin counters
echo "abc" | scribec cat test
tail -f /var/log/nginx/access.log | scribec std_logger access_log
```
