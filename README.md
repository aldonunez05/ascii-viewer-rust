# ascii-viewer-rust

A 3D model viewer for the terminal. It loads a Wavefront `.obj` file and
draws it as a spinning, shaded ASCII-art model:

```
            @@@@@@@
        @@@@@@@@@@@@@@@
     %%%@@@@@@@@@@@@@@@@@@
     %%%%%%%%@@@@@@@@@@@::
     %%%%%%%%%%%%%%%::::::
     %%%%%%%%%%%%%::::::::
      %%%%%%%%%%%:::::::
          %%%%%%%::::
```

Written as a project for learning Rust. The only dependency is
[`glam`](https://docs.rs/glam), which provides vectors and matrices.

## Running

```sh
cd src
cargo run                          # spins the default model, ../models/cube.obj
cargo run -- path/to/model.obj     # any other .obj file
cargo test                         # run the unit tests
```

Press **Ctrl+C** to quit.

The default path `../models/cube.obj` is relative to the directory you run
from, so run the default from inside `src/`. When you pass your own path, it
can be absolute, or relative to wherever you run the command.

## How it works

Each frame goes through the same pipeline. Every stage lives in its own file:

```
 .obj file
    │  model.rs           parse v / f lines into vertices and faces
    ▼
 Model { vertices, faces }
    │  model.rs           normalize(): center at the origin, fit in [-1, 1]
    │  model.rs           triangles(): split polygons, using triangularization.rs
    ▼
 list of triangles (vertex indices)
    │  camera.rs          rotate, place the camera, apply perspective
    ▼
 screen-space points (column, row, depth)
    │  shading.rs         face normal · light direction → one character
    │  raster.rs          fill each triangle, keep the nearest surface
    ▼
 canvas.rs                grid of characters plus a depth buffer
    │  main.rs            print, rotate a little, repeat about 30× per second
    ▼
 terminal
```

Loading, normalizing and triangulating happen **once**, at startup. Only the
camera changes from frame to frame, so the loop just re-projects and redraws
the same triangles.

## The files

### `main.rs`: the conductor

`main.rs` connects the other modules and contains no math of its own.

- **Setup:** reads the path from the command line, loads the model,
  normalizes it, triangulates it, and creates the canvas and camera.
- **`render()`:** draws one frame. For each triangle it computes the shading
  character, projects the 3 corners to the screen and fills the triangle.
- **Loop:** clears the canvas, renders, prints, increases `cam.yaw` and
  sleeps 33 ms.
- **ANSI escape codes:** `\x1b[2J` clears the terminal once at startup.
  `\x1b[H` moves the cursor to the top-left corner before each frame, so
  every frame overwrites the previous one instead of scrolling.
- **`main` returns `Result<(), String>`**, so a missing or broken file prints
  a clean error message instead of panicking.

### `model.rs`: the 3D data

- **`Model`** holds three lists: `vertices` (points in 3D), `faces` and
  `materials`.
- **`Face`** stores **indices** into `vertices`, not copies of the points. A
  cube corner shared by 3 faces is stored once.
- **`load_obj()`** reads `v x y z` lines into vertices and `f a b c ...` lines
  into faces. OBJ indices start at 1, so it subtracts 1. In entries like
  `7/2/5`, only the vertex number (`7`) is used. It ignores every other kind
  of line (normals, texture coordinates, materials, comments).
- **`normalize()`** uses `bounds()` to move the model's center to the origin
  and scale it so its largest side spans -1 to 1. That lets one camera setup
  work for any file, whether the model is tiny or huge.
- **`triangles()`** converts each face into triangles with `triangularize`,
  then maps the per-face indices back to model-wide vertex indices.

### `triangularization.rs`: polygons to triangles

OBJ faces can have any number of corners, but the rasterizer only draws
triangles. `triangularize()` takes a polygon's points and returns triangle
indices, 3 per triangle.

1. Find the plane the polygon lies in and flatten the points onto 2D
   coordinates in that plane.
2. Work out whether the points go clockwise or counter-clockwise (signed
   area).
3. Clip off convex corners as triangles one at a time. If another vertex
   lies inside the corner's triangle, split the polygon along a diagonal to
   that vertex and recurse on both halves.

This also handles non-convex shapes such as an L shape. See the tests at the
bottom of the file.

### `camera.rs`: 3D to screen

A `Camera` has `yaw` and `pitch` (rotation), `distance` (how far away the
camera is) and `fov` (field of view).

`matrix()` combines three 4×4 transforms into one matrix. They apply right to
left:

| Transform | Built with | What it does |
|---|---|---|
| model | `rotation()` | spin the object |
| view | `look_at_rh` | camera at `(0, 0, distance)`, looking at the origin |
| projection | `perspective_rh` | perspective: things farther away look smaller |

`project()` applies the matrix to a point, divides by `w` (the perspective
divide) and maps the result from [-1, 1] to columns and rows. The row is
flipped because screen row 0 is at the top. The returned `z` is the distance
from the camera, used for the depth test.

Terminal characters are about twice as tall as they are wide. The aspect
ratio uses `height * 2` to compensate, so a cube doesn't look stretched.

### `shading.rs`: brightness to characters

- **`normal()`:** the cross product of two triangle edges is a vector
  sticking straight out of the face. Corner order matters: counter-clockwise
  as seen from outside gives an outward normal.
- **`shade()`:** the dot product of the normal with the light direction gives
  the brightness. 1 means the face points straight at the light, and 0 or
  less means it faces away. The brightness picks a character from the ramp
  `. : - = + * # % @`, from dark to bright.

Normals are rotated with the model (`transform_vector3`) but the light stays
fixed, so faces change brightness as the model spins.

### `raster.rs`: filling triangles

`draw_triangle()` takes 3 points in screen space:

1. Compute the triangle's bounding box, clamped to the canvas.
2. For each cell center in the box, evaluate the three **edge functions**.
   Each is a 2D cross product that says which side of an edge the point is
   on.
3. Dividing each edge function by the triangle's area gives **barycentric
   weights**. If all three are ≥ 0, the cell is inside the triangle.
4. Blend the corners' depths with those weights and call `canvas.set()`.

### `canvas.rs`: the framebuffer

- Holds a `width × height` grid stored as one flat `Vec<char>`. Cell
  `(x, y)` is at index `y * width + x`.
- Has a matching `Vec<f32>` **depth buffer**.
- **`set()`** ignores points outside the canvas, and only writes a character
  if it's closer than what's already in that cell. Because of this,
  triangles can be drawn in any order and the nearest surface still ends up
  on top.
- **`clear()`** resets both buffers each frame.
- **`impl Display`** makes `print!("{canvas}")` print the grid row by row.


## Rust concepts used, by file

| Concept | Where |
|---|---|
| structs, `impl`, `new()`, `#[derive]` | `model.rs`, `canvas.rs`, `camera.rs` |
| `&self` / `&mut self` / ownership moves | `model.rs` (`add_face` takes the `Vec`) |
| `Option`, `Result`, `?`, `let ... else` | `model.rs` (`load_obj`, `bounds`, `normalize`) |
| iterators, closures, `collect`, `chunks_exact` | `model.rs` (`triangles`) |
| private fields and encapsulation | `canvas.rs` (`chars`, `depth`) |
| traits (`impl Display`) | `canvas.rs` |
| unit tests with `#[cfg(test)]` | `canvas.rs`, `triangularization.rs` |
| `const` arrays, why `str` can't be indexed | `shading.rs` |
| `as` casts, ranges (`..=`) | `raster.rs` |
| `main` returning `Result`, `std::env::args`, the `Write` trait | `main.rs` |

