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

<!-- Introduction/README.md begins -->

## Chapter 1 {#h1__1__chapter-1}

This is an example chapter. It has many subchapters like [Subsection](#h1__1__2__subsection), [Level 10 section](Introduction/Level10Section.md#   ) and
[Level 9 section]( <Introduction/./Level9Section.md#>
'Level9section'). The book has also [conclusion](#h2__1__conclusion 'Conclusion').
The annexes can be found in [Annexes][suffix].

[suffix]: #h3__1__suffix-chapter "Annexes"

More features can be found in [things](#h1__1__1__things) chapter. Links to sections in other files also work:
[Subsection of the subsection](   #h1__1__2__1__subsection-of-the-subsection   ).

### Things {#h1__1__1__things .unnumbered .unlisted}

And here a LaTeX math formula: $\int x dx = \frac{x^2}{2} + C$ and also a block LaTeX math formula:
$$\mu = \frac{1}{N} \sum_{i=0} x_i$$

And also an image with a bit of HTML:
<div class="image_env">HTML <b>content</b></div>

![My image alt](Introduction/../test_image.png "My image title")

<span class="custom_caption">My custom image caption</span>

And this is a PlantUML diagram:

![](data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiBjb250ZW50U3R5bGVUeXBlPSJ0ZXh0L2NzcyIgaGVpZ2h0PSIxNzBweCIgcHJlc2VydmVBc3BlY3RSYXRpbz0ibm9uZSIgc3R5bGU9IndpZHRoOjU2cHg7aGVpZ2h0OjE3MHB4O2JhY2tncm91bmQ6I0ZGRkZGRjsiIHZlcnNpb249IjEuMSIgdmlld0JveD0iMCAwIDU2IDE3MCIgd2lkdGg9IjU2cHgiIHpvb21BbmRQYW49Im1hZ25pZnkiPjxkZWZzLz48Zz48IS0tTUQ1PVthNDZlZTVjZTBjZWIwMzgwY2Y0NDZhMDhhMDU2ZGNlNF0KY2xhc3MgQS0tPjxnIGlkPSJlbGVtX0EiPjxyZWN0IGZpbGw9IiNGMUYxRjEiIGhlaWdodD0iNDgiIGlkPSJBIiByeD0iMi41IiByeT0iMi41IiBzdHlsZT0ic3Ryb2tlOiMxODE4MTg7c3Ryb2tlLXdpZHRoOjAuNTsiIHdpZHRoPSI0MSIgeD0iNy41IiB5PSI3Ii8+PGVsbGlwc2UgY3g9IjIyLjUiIGN5PSIyMyIgZmlsbD0iI0FERDFCMiIgcng9IjExIiByeT0iMTEiIHN0eWxlPSJzdHJva2U6IzE4MTgxODtzdHJva2Utd2lkdGg6MS4wOyIvPjxwYXRoIGQ9Ik0yNS40Njg4LDI4LjY0MDYgUTI0Ljg5MDYsMjguOTM3NSAyNC4yNSwyOS4wNzgxIFEyMy42MDk0LDI5LjIzNDQgMjIuOTA2MywyOS4yMzQ0IFEyMC40MDYzLDI5LjIzNDQgMTkuMDc4MSwyNy41OTM4IFExNy43NjU2LDI1LjkzNzUgMTcuNzY1NiwyMi44MTI1IFExNy43NjU2LDE5LjY4NzUgMTkuMDc4MSwxOC4wMzEzIFEyMC40MDYzLDE2LjM3NSAyMi45MDYzLDE2LjM3NSBRMjMuNjA5NCwxNi4zNzUgMjQuMjUsMTYuNTMxMyBRMjQuOTA2MywxNi42ODc1IDI1LjQ2ODgsMTYuOTg0NCBMMjUuNDY4OCwxOS43MDMxIFEyNC44NDM4LDE5LjEyNSAyNC4yNSwxOC44NTk0IFEyMy42NTYzLDE4LjU3ODEgMjMuMDMxMywxOC41NzgxIFEyMS42ODc1LDE4LjU3ODEgMjEsMTkuNjU2MyBRMjAuMzEyNSwyMC43MTg4IDIwLjMxMjUsMjIuODEyNSBRMjAuMzEyNSwyNC45MDYzIDIxLDI1Ljk4NDQgUTIxLjY4NzUsMjcuMDQ2OSAyMy4wMzEzLDI3LjA0NjkgUTIzLjY1NjMsMjcuMDQ2OSAyNC4yNSwyNi43ODEzIFEyNC44NDM4LDI2LjUgMjUuNDY4OCwyNS45MjE5IEwyNS40Njg4LDI4LjY0MDYgWiAiIGZpbGw9IiMwMDAwMDAiLz48dGV4dCBmaWxsPSIjMDAwMDAwIiBmb250LWZhbWlseT0ic2Fucy1zZXJpZiIgZm9udC1zaXplPSIxNCIgbGVuZ3RoQWRqdXN0PSJzcGFjaW5nIiB0ZXh0TGVuZ3RoPSI5IiB4PSIzNi41IiB5PSIyNy44NDY3Ij5BPC90ZXh0PjxsaW5lIHN0eWxlPSJzdHJva2U6IzE4MTgxODtzdHJva2Utd2lkdGg6MC41OyIgeDE9IjguNSIgeDI9IjQ3LjUiIHkxPSIzOSIgeTI9IjM5Ii8+PGxpbmUgc3R5bGU9InN0cm9rZTojMTgxODE4O3N0cm9rZS13aWR0aDowLjU7IiB4MT0iOC41IiB4Mj0iNDcuNSIgeTE9IjQ3IiB5Mj0iNDciLz48L2c+PCEtLU1ENT1bNjljNDUxYzc1MWM0MTRlMzc1NDliNzgyMTkwZmVmNDZdCmNsYXNzIEItLT48ZyBpZD0iZWxlbV9CIj48cmVjdCBmaWxsPSIjRjFGMUYxIiBoZWlnaHQ9IjQ4IiBpZD0iQiIgcng9IjIuNSIgcnk9IjIuNSIgc3R5bGU9InN0cm9rZTojMTgxODE4O3N0cm9rZS13aWR0aDowLjU7IiB3aWR0aD0iNDIiIHg9IjciIHk9IjExNSIvPjxlbGxpcHNlIGN4PSIyMiIgY3k9IjEzMSIgZmlsbD0iI0FERDFCMiIgcng9IjExIiByeT0iMTEiIHN0eWxlPSJzdHJva2U6IzE4MTgxODtzdHJva2Utd2lkdGg6MS4wOyIvPjxwYXRoIGQ9Ik0yNC45Njg4LDEzNi42NDA2IFEyNC4zOTA2LDEzNi45Mzc1IDIzLjc1LDEzNy4wNzgxIFEyMy4xMDk0LDEzNy4yMzQ0IDIyLjQwNjMsMTM3LjIzNDQgUTE5LjkwNjMsMTM3LjIzNDQgMTguNTc4MSwxMzUuNTkzOCBRMTcuMjY1NiwxMzMuOTM3NSAxNy4yNjU2LDEzMC44MTI1IFExNy4yNjU2LDEyNy42ODc1IDE4LjU3ODEsMTI2LjAzMTMgUTE5LjkwNjMsMTI0LjM3NSAyMi40MDYzLDEyNC4zNzUgUTIzLjEwOTQsMTI0LjM3NSAyMy43NSwxMjQuNTMxMyBRMjQuNDA2MywxMjQuNjg3NSAyNC45Njg4LDEyNC45ODQ0IEwyNC45Njg4LDEyNy43MDMxIFEyNC4zNDM4LDEyNy4xMjUgMjMuNzUsMTI2Ljg1OTQgUTIzLjE1NjMsMTI2LjU3ODEgMjIuNTMxMywxMjYuNTc4MSBRMjEuMTg3NSwxMjYuNTc4MSAyMC41LDEyNy42NTYzIFExOS44MTI1LDEyOC43MTg4IDE5LjgxMjUsMTMwLjgxMjUgUTE5LjgxMjUsMTMyLjkwNjMgMjAuNSwxMzMuOTg0NCBRMjEuMTg3NSwxMzUuMDQ2OSAyMi41MzEzLDEzNS4wNDY5IFEyMy4xNTYzLDEzNS4wNDY5IDIzLjc1LDEzNC43ODEzIFEyNC4zNDM4LDEzNC41IDI0Ljk2ODgsMTMzLjkyMTkgTDI0Ljk2ODgsMTM2LjY0MDYgWiAiIGZpbGw9IiMwMDAwMDAiLz48dGV4dCBmaWxsPSIjMDAwMDAwIiBmb250LWZhbWlseT0ic2Fucy1zZXJpZiIgZm9udC1zaXplPSIxNCIgbGVuZ3RoQWRqdXN0PSJzcGFjaW5nIiB0ZXh0TGVuZ3RoPSIxMCIgeD0iMzYiIHk9IjEzNS44NDY3Ij5CPC90ZXh0PjxsaW5lIHN0eWxlPSJzdHJva2U6IzE4MTgxODtzdHJva2Utd2lkdGg6MC41OyIgeDE9IjgiIHgyPSI0OCIgeTE9IjE0NyIgeTI9IjE0NyIvPjxsaW5lIHN0eWxlPSJzdHJva2U6IzE4MTgxODtzdHJva2Utd2lkdGg6MC41OyIgeDE9IjgiIHgyPSI0OCIgeTE9IjE1NSIgeTI9IjE1NSIvPjwvZz48IS0tTUQ1PVs1ZTJlZWM2MmE1MGIyNmI2YjBkYjZjMTJlNGVlMjg5Y10KbGluayBBIHRvIEItLT48ZyBpZD0ibGlua19BX0IiPjxwYXRoIGNvZGVMaW5lPSIxIiBkPSJNMjgsNTUgQzI4LDY2LjU5IDI4LDgxLjAzIDI4LDk0LjYgIiBmaWxsPSJub25lIiBpZD0iQS10by1CIiBzdHlsZT0ic3Ryb2tlOiMxODE4MTg7c3Ryb2tlLXdpZHRoOjEuMDsiLz48cG9seWdvbiBmaWxsPSJub25lIiBwb2ludHM9IjM1LDk0LjY4LDI4LDExNC42OCwyMSw5NC42OCwzNSw5NC42OCIgc3R5bGU9InN0cm9rZTojMTgxODE4O3N0cm9rZS13aWR0aDoxLjA7Ii8+PC9nPjwvZz48L3N2Zz4=)



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


<!-- Introduction/README.md ends -->


<!-- Introduction/Subsection.md begins -->

### Subsection {#h1__1__2__subsection}

In this subsection we write text.

#### Subsection  of the subsection {#h1__1__2__1__subsection-of-the-subsection .unnumbered .unlisted}


This is a example subsection.

###### 4th level header {#h1__1__2__1__1__1__4th-level-header .unnumbered .unlisted}

**6th level header**



<!-- Introduction/Subsection.md ends -->


<!-- Introduction/Subsubsection.md begins -->

#### Subsubsection {#h1__1__2__2__subsubsection}

##### Another header {#h1__1__2__2__1__another-header .unnumbered .unlisted}
Deeper level!


<!-- Introduction/Subsubsection.md ends -->


<!-- Introduction/Level4Section.md begins -->

##### Level4 Section {#h1__1__2__2__2__level4-section}
Level4 Section contents.


<!-- Introduction/Level4Section.md ends -->


<!-- Introduction/Level5Section.md begins -->

###### Level5 Section {#h1__1__2__2__2__1__level5-section}
Level5 Section contents.


<!-- Introduction/Level5Section.md ends -->


<!-- Introduction/Level6Section.md begins -->

**Level6 Section**

Level6 Section contents.


<!-- Introduction/Level6Section.md ends -->


<!-- Introduction/Level7Section.md begins -->

**Level7 Section**

Level7 Section contents.


<!-- Introduction/Level7Section.md ends -->


<!-- Introduction/Level8Section.md begins -->

**Level8 Section**

Level8 Section contents.


<!-- Introduction/Level8Section.md ends -->


<!-- Introduction/Level9Section.md begins -->

**Level9 Section**

Level9 Section contents.


<!-- Introduction/Level9Section.md ends -->


<!-- Introduction/Level10Section.md begins -->

**Level10 Section**

Level10 Section contents.


<!-- Introduction/Level10Section.md ends -->


# Part 3 {#h2__part-3}

<!-- Conclusion.md begins -->

## Conclusion {#h2__1__conclusion}

This works like a charm!


<!-- Conclusion.md ends -->

# Annexes {#h3__annexes}

<!-- suffix.md begins -->

## Suffix chapter {#h3__1__suffix-chapter}

This is a suffix!


<!-- suffix.md ends -->
