

Developer Notes
================================================================================




Vibe coding prompts
================================================================================


[2025.10.15] {Making a script to chop up the sprites}
--------------------------------------------------------------------------------

The file `blinktwocolumn.png` contains a sprite sheet. Let's write a
script to break it up. This image is rather large, 3840x10416. It
consists of 7 wide rectangualr sprites that cover the whole width
(3840). There are bands of transparent pixels separating the sprites.

The script needs to split up the sprites by identifying their edges
based on the transparent pixels and cutting each sprite out into a
separate file. We can take a flag `-o name` and name the output
sprites name01, name02 etc.

Keep working until you think you have a working solution, taking care
to read the input file but not damage it. You can make and delete as
many temporary output directories and files as you like. When you
think it works, I'll review the output images.

----

$ identify -format "%wx%h" input/blink01.png
3840x1261

----

Now let's add a second script. Take an input directory such as
`input/` which contains a series of PNGs. We want to split each and
resize them. First, split the image into "left and right" halves,
which will be 1920 wide.

Then, for each resulting half, resize it to avertical height of 800
pixels. Finally, crop symetrically from both left/right edges of the
image to cut it down to 480 width. The result for N input impages will
be 2*N images that are 480x800. Direct the outputs to an output
directory (specified again with -o), and for each original image
named, e.g. `foo01.png` create outputs `foo01L.png` and `foo01R.png`
for the left and right sides.

--- 

We're going to replay the (partial) program in this folder with a
simpler one.  Rather than trying to use the complicated bevy game
engine, and use texture atlases to load a sprite sheet, let's create
an animation in the simplest possible way.

Use the simplest widely used libraries available, ideally with simpler
types (fewer traits, polymorphism, complicated callbacks).

The directory `assets/blink_one_eye/blink*.png` contains a series of
frames. Load these all into memory in an array. Then, create a simple
animation by opening a window at the same resolution as those images
(480x800). Play an animation by playing frames 1..N, leaving each
frame up for 100ms, and then reversing the frames rather than looping
(e.g. frames 1,2,3,4,5,6,5,4,3,2,1,2...).
