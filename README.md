## Closure plots

Simple script written for our [SoME4 video submission](https://youtu.be/wzAYGgzUtNA?si=HP2VQNxV8qdkuadv) to render closure plots of any number system.

Look at the example closure plots shown in the video in more detail [here](https://plots.eliavaux.com).

You can pass *any* type that implements the Float trait into `closure_plot_3d()`.
The operation to plot the closure can be changed to whatever you like.
You could plot complex formulas, though you might need to change the number system to something
more precise than 64 bit floats (like 128 bit floats, or even astrofloats).

## Usage

Run `cargo run --release --bin image` in your terminal to generate some images.

You can change the image **file path**, the **resolution**, closure **operation** and **data types**
to be compared inside `src/bin/image.rs`.

## Info

At the highest resolution of `16`, images get too big to open with a regular image viewer,
so you might want to convert them into Deep Zoom Images, or a comparable format.
Also, my code takes up about 80GB of RAM on the highest resolution.
I could probably reduce this to around 12GB if I refactored some stuff, but this was really
only a one-off script for the video, so I don't see the need for that right now.
If you want to use my code and you need it to be memory efficient, open an issue.
I'll try to fix it as soon as I get the time.

The plots are parallelized using rayon and take about 5 seconds to run on highest resolution on
our uni's compute server.
If you don't want that, change the `.par_iter()` inside the `closure_plot_3d()` function to `.iter()`.
