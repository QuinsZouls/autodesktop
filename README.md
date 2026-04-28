# autodesktop

CLI en Rust para captura de pantalla con grid de coordenadas personalizable y control remoto de input (mouse/teclado).

## Instalacion

### Desde el repo

```bash
git clone git@github.com:QuinsZouls/autodesktop.git
cd autodesktop
cargo build --release
```

El binario queda en `target/release/autodesktop`.

### Con Cargo (una vez publicado)

```bash
cargo install autodesktop
```

## Uso

### Info de displays

```bash
# Listar monitores disponibles
autodesktop info
```

Salida de ejemplo:
```
Available displays:

  [1] Built-in Retina Display [PRIMARY] (1680x1050)

Tip: Use --display <index> to select a specific monitor
Tip: Use --grid 100 to add a coordinate grid overlay
```

### Captura de pantalla

```bash
# Captura basica del monitor principal
autodesktop capture

# Captura con grid cada 100px
autodesktop capture --grid 100

# Grid con color personalizado y labels de coordenadas
autodesktop capture --grid 100 --grid-color yellow --grid-labels

# Grid fino con opacidad reducida
autodesktop capture --grid 50 --grid-color "#00FF00" --grid-opacity 0.3

# Capturar una region especifica
autodesktop capture --region 0,0,800,600 --grid 100 --grid-labels

# Capturar monitor especifico
autodesktop capture --display 1 --grid 200

# Guardar en formato diferente
autodesktop capture -o screenshot.jpg --grid 100
autodesktop capture -o capture.bmp --grid 50
```

### Control del mouse

```bash
# Mover cursor a posicion absoluta
autodesktop mouse move 500 300

# Click en posicion actual
autodesktop mouse click
autodesktop mouse click right
autodesktop mouse click middle

# Click en coordenadas especificas
autodesktop mouse click-at 500 300
autodesktop mouse click-at 800 600 right

# Doble click
autodesktop mouse double-click 500 300

# Arrastrar (drag & drop)
autodesktop mouse drag 100 100 500 500

# Scroll (horizontal, vertical)
autodesktop mouse scroll 0 -3      # Scroll hacia abajo
autodesktop mouse scroll 0 3       # Scroll hacia arriba
autodesktop mouse scroll 5 0       # Scroll horizontal
```

### Control del teclado

```bash
# Escribir texto
autodesktop keyboard type "Hola mundo"
autodesktop keyboard type "https://example.com"

# Pulsar tecla individual
autodesktop keyboard key enter
autodesktop keyboard key escape
autodesktop keyboard key tab
autodesktop keyboard key space
autodesktop keyboard key f1

# Combinaciones de teclas
autodesktop keyboard combo "Ctrl+C"
autodesktop keyboard combo "Cmd+Shift+4"
autodesktop keyboard combo "Alt+Tab"
autodesktop keyboard combo "Cmd+C"
```

## Opciones de grid

| Opcion | Descripcion | Default |
|--------|-------------|---------|
| `--grid <PX>` | Espaciado del grid en pixeles | Desactivado |
| `--grid-color <COLOR>` | Color de las lineas | red |
| `--grid-opacity <0-1>` | Opacidad de las lineas | 0.6 |
| `--grid-labels` | Mostrar coordenadas en bordes | false |
| `--label-size <PX>` | Tamaño de fuente de labels | 14 |
| `--display <N>` | Indice del monitor a capturar | Principal |
| `--region X,Y,W,H` | Capturar region especifica | Pantalla completa |
| `-o, --output <PATH>` | Archivo de salida | screenshot.png |

### Colores soportados

**Nombres:** red, green, blue, yellow, white, black, cyan, magenta, orange

**Hexadecimal:** `#FF0000`, `#00FF00`, `#0000FF`, `#FFFF00`

**RGB:** `255,0,0`, `0,255,0`, `0,0,255`

### Teclas soportadas

**Especiales:** enter/return, esc/escape, tab, backspace/delete, space, shift, ctrl/control, alt/option, cmd/super/meta/command

**Flechas:** up, down, left, right

**Navegacion:** home, end, pageup, pagedown, delete_key/forward_delete

**Funcion:** f1 a f12

**Simbolos:** plus (+), minus (-), equal (=), comma (,), dot/period (.), slash (/), backslash (\), semicolon (;), quote (')

**Caracteres:** Cualquier caracter ASCII se puede usar directamente con `keyboard key a`, `keyboard key 5`, etc.

## Permisos (macOS)

macOS requiere permisos explicitos para funcionar:

1. **Screen Recording**: Para captura de pantalla (xcap)
2. **Accessibility**: Para control de mouse/teclado (enigo)

### Configurar permisos

1. Abrir `System Settings > Privacy & Security`
2. Ir a `Screen Recording` y habilitar Terminal (o tu terminal preferida)
3. Ir a `Accessibility` y habilitar Terminal
4. Reiniciar la terminal despues de dar permisos

Si no funcionan los permisos, probar:
```bash
# Verificar permisos de accesibilidad
tccutil reset Accessibility
tccutil reset ScreenCapture
```

## Ejemplos de uso practico

### Workflow tipico para remote control

```bash
# 1. Capturar pantalla con grid para ver coordenadas
autodesktop capture --grid 100 --grid-color yellow --grid-labels -o grid.png

# 2. Mover mouse a posicion deseada
autodesktop mouse move 800 400

# 3. Hacer click
autodesktop mouse click

# 4. Escribir texto
autodesktop keyboard type "mi_comando"

# 5. Pulsar enter
autodesktop keyboard key enter
```

### Automatizacion de tareas

```bash
# Abrir Spotlight y buscar
autodesktop keyboard combo "Cmd+Space"
sleep 0.5
autodesktop keyboard type "calculator"
sleep 0.5
autodesktop keyboard key enter

# Captura de una region especifica (como Cmd+Shift+4)
autodesktop capture --region 100,100,400,300 --grid 50 -o region.png
```

## Dependencias

- [xcap](https://github.com/nashaofu/xcap) - Captura de pantalla cross-platform
- [enigo](https://github.com/enigo-rs/enigo) - Simulacion de input
- [clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [image](https://github.com/image-rs/image) - Procesamiento de imagenes
- [imageproc](https://github.com/image-rs/imageproc) - Dibujar sobre imagenes

## Desarrollo

```bash
# Build debug
cargo build

# Build release (optimizado)
cargo build --release

# Ver ayuda completa
cargo run -- --help
cargo run -- capture --help
cargo run -- mouse --help
cargo run -- keyboard --help

# Run tests
cargo test
```

## Licencia

MIT
