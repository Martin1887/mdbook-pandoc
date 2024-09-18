---
title: Test ebook
creator:
- role: author
  text: Test author
- role: illustrator
  text: Test
date: 2022-01-01
lang: en-US
description: This description is a test
publisher: Test publisher
group-position: 0
ibooks:
  version: '1.0'
  iphone-orientation-lock: portrait-only
---



# Part 1 {#h1__part-1}

---
title: Overwritten title
---

## Chapter 1 {#main-chapter .myclass}

This is an example chapter. It has many subchapters like [Subsection](#h1__1__2__subsection), [Level 10 section](Introduction/Level10Section.md#   ) and
[Level 9 section]( <Introduction/./Level9Section.md#>
'Level9section'). The book has also [conclusion](#h2__1__conclusion 'Conclusion').
The annexes can be found in [Annexes][suffix].

[suffix]: #h3__1__suffix-chapter "Annexes"

Links to headings with custom ids is of course also supported: [Chapter 1](#main-chapter).

More features can be found in [things](#h1__1__1__things) chapter. Links to sections in other files also work:
[Subsection of the subsection](   #h1__1__2__1__subsection-of-the-subsection   ).

Shortcuts links also work ([suffix]) and links to missing references report a warning: [suffix1], [suffix1][], [my link][suffix1].

Headings inside code blocks or YAML blocks are not detected:

```python
# A comment
print('Hello world')
```

### Things {#h1__1__1__things .unnumbered .unlisted}

And here a LaTeX math formula with mdBook-style delimiters:
\\(\int x dx = \frac{x^2}{2} + C\\) and also a block LaTeX math formula:
\\[     \mu = \frac{1}{N} \sum_{i=0} x_i     \\]

Markdown common math delimiters (`$inline math$` and `$$display math$$`) also
work: $\int x dx = \frac{x^2}{2} + C\\$, $$\int x dx = \frac{x^2}{2} + C\\$$.

And also an image with a bit of HTML:
<div class="image_env">HTML <b>content</b></div>

![My image alt](Introduction/../test_image.png "My image title")

<span class="custom_caption">My custom image caption</span>

And this is a PlantUML diagram:

![](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiBjb250ZW50U3R5bGVUeXBlPSJ0ZXh0L2NzcyIgaGVpZ2h0PSIxNzBweCIgcHJlc2VydmVBc3BlY3RSYXRpbz0ibm9uZSIgc3R5bGU9IndpZHRoOjU1cHg7aGVpZ2h0OjE3MHB4O2JhY2tncm91bmQ6I0ZGRkZGRjsiIHZlcnNpb249IjEuMSIgdmlld0JveD0iMCAwIDU1IDE3MCIgd2lkdGg9IjU1cHgiIHpvb21BbmRQYW49Im1hZ25pZnkiPjxkZWZzLz48Zz48IS0tY2xhc3MgQS0tPjxnIGlkPSJlbGVtX0EiPjxyZWN0IGZpbGw9IiNGMUYxRjEiIGhlaWdodD0iNDgiIGlkPSJBIiByeD0iMi41IiByeT0iMi41IiBzdHlsZT0ic3Ryb2tlOiMxODE4MTg7c3Ryb2tlLXdpZHRoOjAuNTsiIHdpZHRoPSI0MCIgeD0iNy41IiB5PSI3Ii8+PGVsbGlwc2UgY3g9IjIyLjUiIGN5PSIyMyIgZmlsbD0iI0FERDFCMiIgcng9IjExIiByeT0iMTEiIHN0eWxlPSJzdHJva2U6IzE4MTgxODtzdHJva2Utd2lkdGg6MS4wOyIvPjxwYXRoIGQ9Ik0yMi41NjI1LDE3LjA0NjkgQzE5Ljc5NjksMTcuMDQ2OSAxNy40Mzc1LDE5LjI5NjkgMTcuNDM3NSwyMy4zNzUgQzE3LjQzNzUsMjcuNDIxOSAxOS42NDA2LDI5LjcwMzEgMjIuNTkzOCwyOS43MDMxIEMyNC4yMzQ0LDI5LjcwMzEgMjUuNDg0NCwyOS4wMTU2IDI2LjMxMjUsMjguMTQwNiBMMjUuMTcxOSwyNi43MzQ0IEMyNC4zMTI1LDI3LjM1OTQgMjMuNjcxOSwyNy44MTI1IDIyLjY0MDYsMjcuODEyNSBDMjAuOTM3NSwyNy44MTI1IDE5LjY4NzUsMjYuNjQwNiAxOS42ODc1LDIzLjM3NSBDMTkuNjg3NSwxOS45NTMxIDIwLjk4NDQsMTguOTA2MyAyMi42NzE5LDE4LjkwNjMgQzIzLjQzNzUsMTguOTA2MyAyNC4wNDY5LDE5LjE1NjMgMjQuOTUzMSwxOS44OTA2IEwyNi4xODc1LDE4LjQzNzUgQzI1LjAxNTYsMTcuNSAyNC4xMDk0LDE3LjA0NjkgMjIuNTYyNSwxNy4wNDY5IFogIiBmaWxsPSIjMDAwMDAwIi8+PHRleHQgZmlsbD0iIzAwMDAwMCIgZm9udC1mYW1pbHk9InNhbnMtc2VyaWYiIGZvbnQtc2l6ZT0iMTQiIGxlbmd0aEFkanVzdD0ic3BhY2luZyIgdGV4dExlbmd0aD0iOCIgeD0iMzYuNSIgeT0iMjcuNjkiPkE8L3RleHQ+PGxpbmUgc3R5bGU9InN0cm9rZTojMTgxODE4O3N0cm9rZS13aWR0aDowLjU7IiB4MT0iOC41IiB4Mj0iNDYuNSIgeTE9IjM5IiB5Mj0iMzkiLz48bGluZSBzdHlsZT0ic3Ryb2tlOiMxODE4MTg7c3Ryb2tlLXdpZHRoOjAuNTsiIHgxPSI4LjUiIHgyPSI0Ni41IiB5MT0iNDciIHkyPSI0NyIvPjwvZz48IS0tY2xhc3MgQi0tPjxnIGlkPSJlbGVtX0IiPjxyZWN0IGZpbGw9IiNGMUYxRjEiIGhlaWdodD0iNDgiIGlkPSJCIiByeD0iMi41IiByeT0iMi41IiBzdHlsZT0ic3Ryb2tlOiMxODE4MTg7c3Ryb2tlLXdpZHRoOjAuNTsiIHdpZHRoPSI0MSIgeD0iNyIgeT0iMTE1Ii8+PGVsbGlwc2UgY3g9IjIyIiBjeT0iMTMxIiBmaWxsPSIjQUREMUIyIiByeD0iMTEiIHJ5PSIxMSIgc3R5bGU9InN0cm9rZTojMTgxODE4O3N0cm9rZS13aWR0aDoxLjA7Ii8+PHBhdGggZD0iTTIyLjA2MjUsMTI1LjA0NjkgQzE5LjI5NjksMTI1LjA0NjkgMTYuOTM3NSwxMjcuMjk2OSAxNi45Mzc1LDEzMS4zNzUgQzE2LjkzNzUsMTM1LjQyMTkgMTkuMTQwNiwxMzcuNzAzMSAyMi4wOTM4LDEzNy43MDMxIEMyMy43MzQ0LDEzNy43MDMxIDI0Ljk4NDQsMTM3LjAxNTYgMjUuODEyNSwxMzYuMTQwNiBMMjQuNjcxOSwxMzQuNzM0NCBDMjMuODEyNSwxMzUuMzU5NCAyMy4xNzE5LDEzNS44MTI1IDIyLjE0MDYsMTM1LjgxMjUgQzIwLjQzNzUsMTM1LjgxMjUgMTkuMTg3NSwxMzQuNjQwNiAxOS4xODc1LDEzMS4zNzUgQzE5LjE4NzUsMTI3Ljk1MzEgMjAuNDg0NCwxMjYuOTA2MyAyMi4xNzE5LDEyNi45MDYzIEMyMi45Mzc1LDEyNi45MDYzIDIzLjU0NjksMTI3LjE1NjMgMjQuNDUzMSwxMjcuODkwNiBMMjUuNjg3NSwxMjYuNDM3NSBDMjQuNTE1NiwxMjUuNSAyMy42MDk0LDEyNS4wNDY5IDIyLjA2MjUsMTI1LjA0NjkgWiAiIGZpbGw9IiMwMDAwMDAiLz48dGV4dCBmaWxsPSIjMDAwMDAwIiBmb250LWZhbWlseT0ic2Fucy1zZXJpZiIgZm9udC1zaXplPSIxNCIgbGVuZ3RoQWRqdXN0PSJzcGFjaW5nIiB0ZXh0TGVuZ3RoPSI5IiB4PSIzNiIgeT0iMTM1LjY5Ij5CPC90ZXh0PjxsaW5lIHN0eWxlPSJzdHJva2U6IzE4MTgxODtzdHJva2Utd2lkdGg6MC41OyIgeDE9IjgiIHgyPSI0NyIgeTE9IjE0NyIgeTI9IjE0NyIvPjxsaW5lIHN0eWxlPSJzdHJva2U6IzE4MTgxODtzdHJva2Utd2lkdGg6MC41OyIgeDE9IjgiIHgyPSI0NyIgeTE9IjE1NSIgeTI9IjE1NSIvPjwvZz48IS0tbGluayBBIHRvIEItLT48ZyBpZD0ibGlua19BX0IiPjxwYXRoIGNvZGVMaW5lPSIxIiBkPSJNMjcuNSw1NS4yNiBDMjcuNSw3Mi45NCAyNy41LDc5LjEzIDI3LjUsOTYuNzkgIiBmaWxsPSJub25lIiBpZD0iQS10by1CIiBzdHlsZT0ic3Ryb2tlOiMxODE4MTg7c3Ryb2tlLXdpZHRoOjEuMDsiLz48cG9seWdvbiBmaWxsPSJub25lIiBwb2ludHM9IjI3LjUsMTE0Ljc5LDMzLjUsOTYuNzksMjEuNSw5Ni43OSwyNy41LDExNC43OSIgc3R5bGU9InN0cm9rZTojMTgxODE4O3N0cm9rZS13aWR0aDoxLjA7Ii8+PC9nPjwvZz48L3N2Zz4=)



And this a bit of `code`:

```rust
fn my_rust_fun(name: &str) -> String {
    format!("Hello, {}", name)
}

my_rust_fun("world");
```

```rust
fn no_playable_fun() {
    println!("Hello");
}
```

```rust
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
ibooks:
  version: 4.0
```


### Subsection {#h1__1__2__subsection}
In this subsection we write text.

#### Subsection  of the subsection {#h1__1__2__1__subsection-of-the-subsection .unnumbered .unlisted}

This is a example subsection.

###### 4th level heading {#h1__1__2__1__1__1__4th-level-heading .unnumbered .unlisted}


**6th level heading**



#### Subsubsection {#h1__1__2__2__subsubsection}

##### Another heading but with únicoñe {#h1__1__2__2__1__another-heading-but-with--xc3-xbanico-xc3-xb1e .unnumbered .unlisted}
Deeper level!


##### Level4 Section {#h1__1__2__2__2__level4-section}
Level4 Section contents.


###### Level5 Section {#h1__1__2__2__2__1__level5-section}
Level5 Section contents.



**Level6 Section**

Level6 Section contents.



**Level7 Section**

Level7 Section contents.



**Level8 Section**

Level8 Section contents.



**Level9 Section**

Level9 Section contents.



**Level10 Section**

Level10 Section contents.


# Part 3 {#h2__part-3}

## Conclusion {#h2__1__conclusion}

This works like a charm!

# Annexes {#h3__annexes}

## Suffix chapter {#h3__1__suffix-chapter}

This is a suffix!
