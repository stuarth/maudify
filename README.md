# Maudify - Convert HTML to [Maud](https://maud.lambda.xyz/)

## How to use it

### Installing

Clone the repo, `cargo build`, and put the resulting executable in your `$PATH`

### Running

```sh
echo '<div class="class-a class-b"><span>foo</span></div>' | maudify
div class="class-a class-b" {
    span { "foo" }
}
```
