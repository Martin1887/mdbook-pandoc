# Chapter 1

This is an example chapter. It has many subchapters like [Subsection](Subsection.md), [Level 10 section](Level10Section.md   ) and
[Level 9 section]( <./Level9Section.md>
'Level9section'). The book has also [conclusion](../Conclusion.md 'Conclusion').
The annexes can be found in [Annexes][suffix].

[suffix]: ../suffix.md "Annexes"

More features can be found in [things](#things) chapter. Links to sections in other files also work:
[Subsection of the subsection](   Subsection.md#subsection-of-the-subsection   ).

## Things

And here a LaTeX math formula: \\(\int x dx = \frac{x^2}{2} + C\\) and also a block LaTeX math formula:
\\[     \mu = \frac{1}{N} \sum_{i=0} x_i     \\]

And also an image with a bit of HTML:
<div class="image_env">HTML <b>content</b></div>

![My image alt](../test_image.png "My image title")

<span class="custom_caption">My custom image caption</span>

And this is a PlantUML diagram:

```plantuml
@startuml
A --|> B
@enduml
```

And this a bit of `code`:

```rust
fn my_rust_fun(name: &str) -> String {
    format!("Hello, {}", name)
}

my_rust_fun("world");
```

```rust,noplayground,ignore
fn no_playable_fun() {
    println!("Hello");
}
```
```rust,editable
fn editable_fun() {
    println!("Edit this string if you want");
}
```


```python
def my_python_fun(name: str) -> str:
    'Hello, {}'.format(name)

my_python_fun("world")
```


```
def my_pseudocode_fun(str name):
    print("Hello, {}".format(name))
```

Now a bit of a YAML file is included:
```yaml
{{#include ../../metadata.yml:2:3}}
```