# Mandelbrot

Application generates Mandelbort set and Julia set images. 
By deafault it shows Mandelbrot set in GUI mode (allows to explore Mandelbrot set using mouse). 
With proper arguments it can generate JPG images in CLI mode or display Julia set (more info at --help).

# Technology

Application has been written in Rust language. SDL2 was used to diplay GUI mode and to generate JPG images in CLI mode.

# Things to improve

* Performance could be improved using OpenCL or at least SIMD (for now it's just simple multithreading).
* Coloring could be much better (for now is just 'naive' scaling with iterations and the effect is not that impressive).
* Config could be done using UI (for now you have to edit config.json to change configuration or by command line parameters).

# GUI Mode

Mandelbrot set

![image](https://user-images.githubusercontent.com/35232230/111034589-b40f9880-8416-11eb-82ef-63aea42424d3.png)

Julia Set for c = -0.10 + 0.65i

![image](https://user-images.githubusercontent.com/35232230/111034755-66476000-8417-11eb-82bc-38605b337c07.png)

# CLI Mode

Mandelbrot generated in CLI mode.
Resolution = 2500x1800

Running in CLI mode:

```
PS> .\mandelbrot_x64.exe --cli
Minimum RAM usage for resolution 2500x1800: 18 MB (17.166 MiB)
Press any key to continue...
[==================================================] 100%
Elapsed time: 8 s 73 ms
```
Generated image (open in new tab):
![1615648714178](https://user-images.githubusercontent.com/35232230/111034910-11f0b000-8418-11eb-8ccf-6ae82c09c24c.png)


