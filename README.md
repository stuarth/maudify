# Maudify - Convert HTML to [Maud](https://maud.lambda.xyz/)

## How to use it

### Installing

```sh
cargo install --git https://github.com/stuarth/maudify/
```

### Running

```sh
echo '<div class="class-a class-b"><span>foo</span></div>' | maudify
div class="class-a class-b" {
    span { "foo" }
}
```
