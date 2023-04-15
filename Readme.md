# Pragmatic ways of using Rust in your data project @ PyCon.DE 2023

Writing efficient data pipelines in Python can be tricky. The standard
recommendation is to use vectorized functions implemented in Numpy, Pandas, or
the like. However, what to do, when the processing task does not fit these
libraries? Using plain Python for processing can result in lacking performance,
in particular when handling large data sets.

Rust is a modern, performance-oriented programming language that is already
widely used by the Python community. Augmenting data processing steps with Rust
can result in substantial speed ups. In this talk will present strategies of
using Rust in a larger Python data processing pipeline with a particular focus
on pragmatism and minimizing integration efforts.

Contents:

1. [Slides](Slides.pdf)
2. IO Patterns
   - [Rust](01_io_patterns)
   - [Python](01_io_pattern.ipynb)
3. PDF Parser
   - [Rust](02_pdf_parser)
   - [Python](02_pdf_parser.ipynb)
4. JSON to Arrow
   - [Rust](03_json_to_arrow)
   - [Python](03_json_to_arrow.ipynb)
5. [Summary of results](04_summary.ipynb)

## Datasets

The "Spotify Million Playlist Dataset" was obtained [here][spotify-dataset]. In
accordance with the [license][spotify-license] the data processing code is
licensed under the MIT license.

[spotify-dataset]: https://www.aicrowd.com/challenges/spotify-million-playlist-dataset-challenge
[spotify-license]: https://www.aicrowd.com/challenges/spotify-million-playlist-dataset-challenge/challenge_rules

## License

```
The MIT License (MIT)
Copyright (c) 2023 Christopher Prohm

Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
```
