 # 🔮 Write readable regular expressions

 The crate provides a clean and readable way of writing your regex in the Rust programming language:

<table>
<tr>
<td>
  
Without `pretty_regex`
  
</td>
<td>
  
With `pretty_regex`

</td>
</tr>

<tr>
<td>
  
```
\d{5}(-\d{4})?
```

</td>
<td>
  
```rs
digit() * 5 + just("-") + (digit() * 4).optional()
```

</td>
</tr>
<tr>
<td>
  
```
^(?:\d){4}(?:(?:\-)(?:\d){2}){2}$
```

</td>
<td>
  
```rs
beginning() + (digit() * 4) +
  (just("-") + digit() * 2) * 2 + ending()
```

</td>
</tr>

<tr>
<td>
  
```
rege(x(es)?|xps?)
```

</td>
<td>
  
```rs
just("rege") + (
  just("x") + just("es").optional() |
  just("xp") + just("s").optional()
)
```

</td>
</tr>
<tr>
</table>

# How to use the crate?

To convert a `PrettyRegex` into a regex from `regex` crate, you can call `to_regex` or `to_regex_or_panic`: 

```rs
use pretty_regex::digit;

let regex = (digit() + ascii_alphabetic().optional()).to_regex_or_panic();

assert!(regex.is_match("3"));
```
