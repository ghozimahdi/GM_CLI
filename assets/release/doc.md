## Step 1 Install GnuPG on your pc

```bash
brew install gnupg
```

## Step 2 Setup Variable Environment

### Save variable on your terminal only

```bash
export ENCRYPT_KEY="contact your leader/project manager for this key"

```

### Save variable on your local environment

```bash
echo 'export ENCRYPT_KEY="contact your leader/project manager for this key"' >>~/.zshrc

```

### **Please note that the key will be used to encrypt and decrypt the secret key in this project**

## Step 3 Encrypt or Decrypt

### encrypt

```bash
encrypt-secrets.sh

```

### decrypt

```bash
decrypt-secrets.sh
```

## (Optional) Step 4 clean secrets

```bash
clean-secrets.sh
```

## **Important if you update google-services.json, GoogleService-info.plist or you update secrets in release folder, please do step 3, if necessary do step 4 also**

`© 2022 Ghozi Mahdi
Inspiring Through Code 💻❤️
Find more magic code tricks at:
Website: www.mbahgojol.com
GitHub: github.com/mbahgojol
YouTube: https://www.youtube.com/@mbahgojol`