## Development

Trace uses the [Rust on Nails](https://rust-on-nails.com/) architecture for secure full stack web applications.

## Pulumi Infra Structure as Code

- az login --use-device-code
- az aks get-credentials --name tebi-prod-cluster --resource-group tebi-production
- export ECDH_PRIVATE_KEY=$(openssl pkcs8 -topk8 -nocrypt -in cloak-pulumi.enc.pem)
- cloak pulumi login
- pulumi up --stack prod

## Defence in Depth

### Container

1. All containers are built from `Scratch` minimising the attack surface.

### Authentication

1. All authentication routes  such as `/auth/login` and `/auth/register` are implemented by `barricade` which is a dedicated docker container.

###  Supply Chain

1. Our CI-CD pipelines compiles nodejs dependencies in a container where to minimise the impact of potential of any malware.
1. We use the minimal number of nodejs dependencies that we can and only use libraries from truted sources.
1. We publish a hash of our containers to ensure the container you deploy has not been tampered with.
1. [OPEN] We hash our CSS and JS files and the hashes are compiled into our server code. Files that are tampred with will not load.

### Database Access

1. The authentication Postgres user only has access to the `users` and `sessions` tables.
1. The application Postgres does not have access to the sessions table.

### Private Keys

1. Keys are stored in the browsers local storage on the user machine and are set as non-extractable.

### Encryption



### XSS and 

1. We have the tightest possible content security policy (CSP) effectively cutting off the possibility of an XSS attack unless the server is compromised.