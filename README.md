# pngme

`pngme` is a cli program that can hide/show some message in a PNG file.

## Install

### Pre built

Pre built files for some architectures can be found in [release](https://github.com/lnkkerst/pngme/releases)

### Cargo

```bash
git clone https://github.com/lnkkerst/pngme.git
cargo install --path ./pngme
```

## Usage

```bash
pngme --help
```

## Examples

Encode message to a PNG file

```bash
pngme encode test.png RuSt "hello pngme!" out.png
```

Decode message from a PNG file

```bash
pngme decode out.png RuSt
```

Remove message from a PNG file

```bash
pngme remove out.png RuSt
```

Print all possible messages.

```bash
pngme print out.png
```

## References

- [picklenerd/pngme_book](https://github.com/picklenerd/pngme_book)
