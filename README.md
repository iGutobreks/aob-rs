# AOB-RS

# Example
Here you can see a example of this.
```rust
match memory_parser("00 10 10 0F", 30, "javaw", None, Some("0F 10 10 00")) {
  Ok(q) => println!("scanned {} and changed {}",q.0,q.1),
  Err(e) => println!("an error occurred! {}", e),
}
```
## Explain
<ul>
  <li>
    "00 10 10 0F" is a pattern to scanning.
  </li>
  <li>
    30 is a max address (the limiter of writer values).
  </li>
  <li>
    "javaw" is a process name.
  </li>
  <li>
    None, is a type of struct `Option<T>`. In this case, I have passed this how None, because I cannot want to write a float or integer in memory. If you want do this, you can with this: <a href="#None">None</a>
  </li>
  <li>
    Some("0F 10 10 00") is a array of bytes to write above pattern.
  </li>
</ul>

<h1 id="None"></h1>

```rust
match memory_parser("00 10 10 0F", 30 "javaw", Some(3.63), None)) {
  Ok(q) => println!("scanned {} and changed {}",q.0,q.1),
  Err(e) => println!("an error occurred! {}",e),
}
```
