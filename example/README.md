# simdnoise-example

## Building

There is one executable that can generate all 5 variants.

```bash
Cargo build --release
./target/release/simdnoise-example --help
```

```text
Example app that uses SIMD Noise

Usage: simdnoise-example [OPTIONS] <COMMAND>

Commands:
  cellular    
  fbm         
  ridge       
  turbulence  
  gradient    
  help        Print this message or the help of the given subcommand(s)

Options:
  -w, --width <WIDTH>          The width of the generated image [default: 1920]
      --height <HEIGHT>        The height of the generated image [default: 1080]
      --depth <DEPTH>          The z dimension of the generated image [default: 1]
      --time <TIME>            The w dimension of the generated image [default: 5]
      --dimension <DIMENSION>  The number of dimensions of the generated noise [default: three] [possible values: one, two, three, four]
      --seed <SEED>            The initial seed value [default: 8]
      --offset-x <OFFSET_X>    Use an offset for the first dimension
      --offset-y <OFFSET_Y>    Use an offset for the second dimension
      --offset-z <OFFSET_Z>    Use an offset for the third dimension
      --offset-w <OFFSET_W>    Use an offset for the fourth dimension
  -h, --help                   Print help
  -V, --version                Print version
```

## Usage

### Cellular

```bash
./target/release/simdnoise-example cellular--help
```

```
Usage: simdnoise-example cellular [OPTIONS]

Options:
      --distance <DISTANCE>    The distance function [default: euclidean] [possible values: euclidean, manhattan, natural]
  -w, --width <WIDTH>          The width of the generated image [default: 1920]
  -f, --frequency <FREQUENCY>  [default: 1.2]
      --height <HEIGHT>        The height of the generated image [default: 1080]
      --depth <DEPTH>          The z dimension of the generated image [default: 1]
  -j, --jitter <JITTER>        [default: 1.2]
      --time <TIME>            The w dimension of the generated image [default: 5]
      --dimension <DIMENSION>  The number of dimensions of the generated noise [default: three] [possible values: one, two, three, four]
      --seed <SEED>            The initial seed value [default: 8]
      --offset-x <OFFSET_X>    Use an offset for the first dimension
      --offset-y <OFFSET_Y>    Use an offset for the second dimension
      --offset-z <OFFSET_Z>    Use an offset for the third dimension
      --offset-w <OFFSET_W>    Use an offset for the fourth dimension
  -h, --help                   Print help
```

### FBM
```bash
./target/release/simdnoise-example fbm --help
```

```
Usage: simdnoise-example fbm [OPTIONS]

Options:
  -f, --frequency <FREQUENCY>    [default: 1.2]
  -w, --width <WIDTH>            The width of the generated image [default: 1920]
      --height <HEIGHT>          The height of the generated image [default: 1080]
  -l, --lacunarity <LACUNARITY>  [default: 0.5]
      --depth <DEPTH>            The z dimension of the generated image [default: 1]
  -g, --gain <GAIN>              [default: 2]
  -o, --octaves <OCTAVES>        [default: 3]
      --time <TIME>              The w dimension of the generated image [default: 5]
      --dimension <DIMENSION>    The number of dimensions of the generated noise [default: three] [possible values: one, two, three, four]
      --seed <SEED>              The initial seed value [default: 8]
      --offset-x <OFFSET_X>      Use an offset for the first dimension
      --offset-y <OFFSET_Y>      Use an offset for the second dimension
      --offset-z <OFFSET_Z>      Use an offset for the third dimension
      --offset-w <OFFSET_W>      Use an offset for the fourth dimension
  -h, --help                     Print help
```

### Ridge

```bash
./target/release/simdnoise-example ridge --help
```

```
Usage: simdnoise-example ridge [OPTIONS]

Options:
  -f, --frequency <FREQUENCY>    [default: 1.2]
  -w, --width <WIDTH>            The width of the generated image [default: 1920]
      --height <HEIGHT>          The height of the generated image [default: 1080]
  -l, --lacunarity <LACUNARITY>  [default: 0.5]
      --depth <DEPTH>            The z dimension of the generated image [default: 1]
  -g, --gain <GAIN>              [default: 2]
  -o, --octaves <OCTAVES>        [default: 3]
      --time <TIME>              The w dimension of the generated image [default: 5]
      --dimension <DIMENSION>    The number of dimensions of the generated noise [default: three] [possible values: one, two, three, four]
      --seed <SEED>              The initial seed value [default: 8]
      --offset-x <OFFSET_X>      Use an offset for the first dimension
      --offset-y <OFFSET_Y>      Use an offset for the second dimension
      --offset-z <OFFSET_Z>      Use an offset for the third dimension
      --offset-w <OFFSET_W>      Use an offset for the fourth dimension
  -h, --help                     Print help
```

### Turbulance

```bash
./target/release/simdnoise-example turbulence--help
```

```
Usage: simdnoise-example turbulence [OPTIONS]

Options:
  -f, --frequency <FREQUENCY>    [default: 1.2]
  -w, --width <WIDTH>            The width of the generated image [default: 1920]
      --height <HEIGHT>          The height of the generated image [default: 1080]
  -l, --lacunarity <LACUNARITY>  [default: 0.5]
      --depth <DEPTH>            The z dimension of the generated image [default: 1]
  -g, --gain <GAIN>              [default: 2]
  -o, --octaves <OCTAVES>        [default: 3]
      --time <TIME>              The w dimension of the generated image [default: 5]
      --dimension <DIMENSION>    The number of dimensions of the generated noise [default: three] [possible values: one, two, three, four]
      --seed <SEED>              The initial seed value [default: 8]
      --offset-x <OFFSET_X>      Use an offset for the first dimension
      --offset-y <OFFSET_Y>      Use an offset for the second dimension
      --offset-z <OFFSET_Z>      Use an offset for the third dimension
      --offset-w <OFFSET_W>      Use an offset for the fourth dimension
  -h, --help                     Print help
```

### Gradient

```bash
./target/release/simdnoise-example gradient --help
```

```text
Usage: simdnoise-example gradient [OPTIONS]

Options:
  -w, --width <WIDTH>          The width of the generated image [default: 1920]
      --height <HEIGHT>        The height of the generated image [default: 1080]
      --depth <DEPTH>          The z dimension of the generated image [default: 1]
      --time <TIME>            The w dimension of the generated image [default: 5]
      --dimension <DIMENSION>  The number of dimensions of the generated noise [default: three] [possible values: one, two, three, four]
      --seed <SEED>            The initial seed value [default: 8]
      --offset-x <OFFSET_X>    Use an offset for the first dimension
      --offset-y <OFFSET_Y>    Use an offset for the second dimension
      --offset-z <OFFSET_Z>    Use an offset for the third dimension
      --offset-w <OFFSET_W>    Use an offset for the fourth dimension
  -h, --help                   Print help
```

