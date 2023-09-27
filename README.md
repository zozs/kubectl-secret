# kubectl-secret

Small utility to base64 decode and print kubernetes secrets in the current context and namespace.

No more `kubectl get secret -o yaml` and then manually decoding secret values with `base64 -d`!

## Usage

The examples assume `kubectl-secret` has been installed somewhere in your `PATH`, then `kubectl` will automatically make it available as `kubectl secret`.

### Print all secrets

```
$ kubectl secret

default / secret1
  AUTH_PASS: example_password
  AUTH_USER: someone

default / moviez
  API_KEY: abcdef1234
  PGDATABASE: moviez
  PGHOST: base.example.com
  PGPASSWORD: hellu
  PGUSER: moviez

default / binary-secret
  binary_data: <binary data of length 64>
```

### Print a particular secret

```
$ kubectl secret secret1

default / secret1
  AUTH_PASS: example_password
  AUTH_USER: someone
```
