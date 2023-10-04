# Fractal Generator
Draws a regular polygon of n sides, then repeats a pattern to see if fractals emerge.
## Chaos Game
The chaos game is a method of iteratively generating fractals within shapes. An initial point is selected, and a new point is selected to be a fraction of the way from that point to a randomly selected vertex. By repeating this method from that new point, fractals appear in some shapes.
### Restricted Chaos Game
For shapes where the normal chaos game does not produce a fractal (such as a square), additional restrictions can be put in place to cause the generation of a fractal. One such example would be restricting the choice of vertex to not include the previously selected vertex. Other methods of altering the pattern could be restricting the vertex to not be immediately counter-clockwise of the previously selected vertex, or by allowing the point to jump towards the center of the shape as well.
## Implementation
For this project, the standard chaos game is used for triangles, while the restriction that the vertex cannot be the previously selected vertex is put in place for all shapes with more than 3 sides. In a triangle, this will quickly produce the Sierpinksi triangle. Interesting results also appeared using shapes up to six sides, though past that the patterns weren't as well defined. If more restrictions were applied, more complex patterns could be produced in shapes with more sides. Interestingly, I noticed that using the restricted chaos game on a triangle will produce a very small amount of possible points and will not produce the Sierpinski triangle.

# Usage
sdl2.dll must be in the same folder as fractal-generator.exe or the program will not run!
By default the number of sides is 3, but any other integer can be passed as an argument to generate shapes with more sides.
