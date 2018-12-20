# Certbot Wrapper

A wrapper around the `certbot` command, allowing multiple config files
instead of a single [`cli.ini`](https://certbot.eff.org/docs/using.html#configuration-file).

## Example

The following example will run certbot for each `.ini` file found in `/etc/letsencrypt/conf.d`:

```bash
certbot-wrapper --config-path /etc/letsencrypt/conf.d \
                -- certonly
```

## Full Setup

### 1. Nginx Config

The following nginx config will allow the certificates to be renewed automatically using the `webroot` authenticator. 

**/etc/nginx/conf.d/example.conf**
```nginx
server {
  listen 80;
  listen [::]:80;

  server_name example.com www.example.com;

  location / {
    return 301 https://$server_name$request_uri;
  }

  include acme_challenge;
}
```

### 2. Certbot Config

See: <https://certbot.eff.org/docs/using.html#configuration-file> for a list of possible options.

**/etc/letsencrypt/conf.d/example.ini**
```ini
rsa-key-size = 4096
email = webmaster@exmaple.com
domains = example.com www.example.com
authenticator = webroot
webroot-path = /usr/share/nginx/html
```

### 3. Generate Certificates

Run `certbot-wrapper -- certonly` to generate the certificates for the first time.

### 4. Add HTTPS Config to Nginx

Now that the certificate is generated, we can configure nginx to accept connections over HTTPS.
Some useful resources are:

- <https://mozilla.github.io/server-side-tls/ssl-config-generator/>
- <https://www.ssllabs.com/index.html>

### 5. Enable auto-renewal

Enable the systemd timer `renew-certificates`, which will try to renew certificates monthly:

```bash
systemctl enable renew-certificates.timer
systemctl start renew-certificates.timer
```
