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
digit()
  .repeats(5)
  .then(
    just("-")
     .then(digit().repeats(4))
     .optional()
  )
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
beginning()
  .then(digit().repeats(4))
  .then(
    just("-")
      .then(digit().repeats(2))
      .repeats(2)
  )
  .then(ending())
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
just("rege")
  .then(one_of(&[
    just("x").then(just("es").optional()),
    just("xp").then(just("s").optional()),
  ]))
```

</td>
</tr>
<tr>
</table>

# How to use the crate?

To convert a `PrettyRegex` struct which is constructed using all these `then`, `one_of`, `beginning`, `digit`, etc. functions into 
a real regex (from `regex` crate), you can call `to_regex` or `to_regex_or_panic`: 

```rs
use pretty_regex::digit;

let regex = digit().to_regex_or_panic();

assert!(regex.is_match("3"));
```
