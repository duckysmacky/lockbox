# LockBox

A fast and secure data encryption program, which focuses on speed, safety and user-friendliness

*The project is still very work-in-progress and the mentioned features could very well change during development*

## About

Lockbox aims to be cross-platform lightweight solution for file encryption

Lockbox is mainly using the **ChaCha20** algorithm with **Poly1305** universal hash function to perform encryption
operations. It proved to be much more safe and fast than the most popular **AES** algorithm

The encrypted files are stored in a custom `.box` file format. This enables the ability to embed additional safety
utilities into the encrypted data, such as checksums, to check file's integrity. It also sets lockbox apart from its
competitors

The general idea behind this project is to standardise file encryption by making a universal, cross-platform and
user-friendly application to secure one's data, replacing clunky and complex old encryption applications

## Features

### Encrypting files

```shell
lockbox box [OPTIONS] [PATH]...
```

Multiple paths can be supplied for multi-file encryption, as well as directories (with optional recursive feature `-R`)

Output files will be encrypted and formatted into a custom `.box` file type with a random UUID as a name. User also
can specify the output location for each file with a `-o` flag

### Decrypting files

```shell
lockbox unbox [OPTIONS] [PATH]...
```

Functions similarly to encryption: support for multiple paths and directories. The original file name can be supplied
instead of a UUID to easily identify files

The input files have to have a `.box` file type. During decryption the program will restore original file name and
extension

## Planned Features

- [ ] Multiple key support
- [ ] User profile system
- [ ] File compression
- [ ] Key backups
- [ ] Algorithm selection
- [ ] GUI Mode
- [ ] Remote key storage

*Plans could change during development*