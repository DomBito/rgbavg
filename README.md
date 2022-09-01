# rgbavg
## Fast and simple colorpicker of an averaged sample.

The rust implementation just calculates the average RGB value of the image `/tmp/rgbavgclip.png` and convert said RGB value to other colorspaces, such as HSL (cylindric form and hexagonal approximation), and YUV (bt601 and rec709).
The clip-rgb is an example of bash script on how to use the compiled `rgbavg` binary to use as a colorpicker. This example uses [scrot](https://github.com/dreamer/scrot) to export a selected area to `/tmp/rgbavgclip.png`, [dunstify](https://github.com/dunst-project/dunst) to notify the output values, and [xclip](https://github.com/astrand/xclip) to parse the RGB value to the clipboard.
