# GitHub Pages Custom Domain Setup Guide

This guide explains how to configure DNS records for the custom domain `idiomatic-rust-snippets.org` to work with GitHub Pages.

## Step 1: Enable GitHub Pages

1. Go to repository Settings → Pages
2. Under "Source", select **GitHub Actions**
3. Under "Custom domain", enter: `idiomatic-rust-snippets.org`
4. Click **Save**

## Step 2: Configure DNS Records

Add the following DNS records to your domain registrar's DNS settings:

### A Records (for apex domain)
Add these **4 A records** pointing to GitHub's IP addresses:

```
Type: A
Name: @ (or leave blank for apex domain)
Value: 185.199.108.153
TTL: 3600 (or default)

Type: A
Name: @ (or leave blank for apex domain)
Value: 185.199.109.153
TTL: 3600 (or default)

Type: A
Name: @ (or leave blank for apex domain)
Value: 185.199.110.153
TTL: 3600 (or default)

Type: A
Name: @ (or leave blank for apex domain)
Value: 185.199.111.153
TTL: 3600 (or default)
```

### AAAA Records (for IPv6 support, optional but recommended)
Add these **4 AAAA records**:

```
Type: AAAA
Name: @ (or leave blank for apex domain)
Value: 2606:50c0:8000::153
TTL: 3600 (or default)

Type: AAAA
Name: @ (or leave blank for apex domain)
Value: 2606:50c0:8001::153
TTL: 3600 (or default)

Type: AAAA
Name: @ (or leave blank for apex domain)
Value: 2606:50c0:8002::153
TTL: 3600 (or default)

Type: AAAA
Name: @ (or leave blank for apex domain)
Value: 2606:50c0:8003::153
TTL: 3600 (or default)
```

### CNAME Record (for www subdomain)
Add a CNAME record for the www subdomain:

```
Type: CNAME
Name: www
Value: saltukalakus.github.io
TTL: 3600 (or default)
```

## Step 3: Get TXT Record for Domain Verification

GitHub requires domain verification via a TXT record. To get your specific verification code:

1. Go to repository Settings → Pages
2. Under "Custom domain", enter: `idiomatic-rust-snippets.org`
3. Click **Save**
4. GitHub will display the TXT record you need to add

The TXT record will look like this:

```
Type: TXT
Name: _github-pages-challenge-saltukalakus
Value: <verification-code-provided-by-github>
TTL: 3600 (or default)
```

**Note:** The verification code is unique and generated when you add the custom domain in GitHub Pages settings. You'll see it displayed in the settings page.

## Step 4: Wait for DNS Propagation

DNS changes can take up to 48 hours to propagate, but typically take effect within a few hours.

You can check DNS propagation status using:
- https://dnschecker.org
- Command line: `dig idiomatic-rust-snippets.org`
- Command line: `nslookup idiomatic-rust-snippets.org`
- Command line: `nslookup -type=txt _github-pages-challenge-saltukalakus.idiomatic-rust-snippets.org gordon.ns.cloudflare.com`
  
## Step 5: Enable HTTPS

Once DNS is configured and verified:

1. Go back to Settings → Pages
2. Check the box for **Enforce HTTPS**
3. GitHub will automatically provision an SSL certificate

## Verification

After everything is configured:
- `https://idiomatic-rust-snippets.org` should load your site
- `https://www.idiomatic-rust-snippets.org` should redirect to the apex domain
- HTTPS should be enabled with a valid SSL certificate

## Troubleshooting

If your site doesn't load:
1. Check that all DNS records are correctly entered
2. Verify domain ownership with the TXT record
3. Wait for DNS propagation (check with dnschecker.org)
4. Check GitHub Pages settings show "DNS check successful"
5. Review GitHub Actions workflow runs for any errors

## Files Added for Custom Domain

- `CNAME` - Contains the custom domain name
- `.nojekyll` - Tells GitHub Pages not to process site with Jekyll
- Updated `.github/workflows/mdbook.yml` - Copies CNAME and .nojekyll to build output
