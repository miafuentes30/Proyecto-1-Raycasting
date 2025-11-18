# Proyecto 1 Raycaster 

Un juego de exploración de mazmorras. Fue implementado con técnicas de raycasting en Rust, utilizando raylib para gráficos.

## Descripción del Juego

Es un juego de exploración en mazmorras oscuras. El jugador debe navegar por laberintos peligrosos, evitar a los diablitos, recolectar todos los cofres y encontrar la salida para avanzar al siguiente nivel.

Video de demostración: https://www.canva.com/design/DAG4bDh6sXA/Or-gI1wo2eDjqyOd17-YtA/edit?utm_content=DAG4bDh6sXA&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton

## Objetivo del Juego

### Para Ganar:
1. **Recolectar TODOS los cofres** del nivel actual
2. **Encontrar la salida** marcada con 'E' (aparece en verde en el minimapa)
3. **Llegar a la salida** para avanzar al siguiente nivel
4. **Completar los 2 niveles** para ganar el juego

### Condiciones de Derrota:
- Si tu vida llega a 0, aparecerá la pantalla de Game Over
- Puedes reintentar presionando 'R' o volver al menú con 'M'

## Controles

### Movimiento:
- **W / Flecha Arriba**: Avanzar
- **S / Flecha Abajo**: Retroceder
- **A**: Moverse a la izquierda (strafe)
- **D**: Moverse a la derecha (strafe)

### Cámara:
- **Ratón (movimiento horizontal)**: Rotar la cámara/vista
- **Flecha Izquierda**: Rotar a la izquierda
- **Flecha Derecha**: Rotar a la derecha

### Acciones:
- **F**: Encender/Apagar linterna
- **ESC**: Abrir menú / Salir del juego

### Menú:
- **1**: Seleccionar Nivel 1
- **2**: Seleccionar Nivel 2
- **ENTER**: Iniciar juego
- **ESC**: Salir del juego

### Pantallas de Fin:
- **R**: Reiniciar nivel (Game Over)
- **M**: Volver al menú principal

## Cómo Ejecutar el Programa

### Requisitos Previos:
- **Rust** (versión 1.70 o superior)
- **Cargo** (incluido con Rust)
- Sistema operativo: Windows, Linux o macOS

### Instalación de Rust:
Si no tienes Rust instalado, descárgalo desde [rustup.rs](https://rustup.rs/)

### Pasos para Ejecutar:

1. **Clonar o descargar el proyecto**:
```bash
   git clone https://github.com/miafuentes30/Proyecto-1-Raycasting.git
   cd Proyecto-1-Raycasting
```

2. **Ejecutar en modo debug** (más lento, útil para desarrollo):
```bash
   cargo run
```

3. **Ejecutar en modo release** (optimizado, recomendado para jugar):
```bash
   cargo run --release
```

4. **Compilar sin ejecutar**:
```bash
   cargo build --release
```
   El ejecutable estará en 	arget/release/

### Solución de Problemas:

- **Error de audio**: Si el audio falla al iniciar, verifica que tu sistema tenga dispositivos de audio disponibles
- **FPS bajos**: Asegúrate de ejecutar en modo --release
- **Texturas no cargan**: Verifica que la carpeta ssets/ esté en el mismo directorio que el ejecutable

## Estructura del Proyecto

```bash
Proyecto-1-Raycasting/
├── src/
│   ├── main.rs           # Punto de entrada, loop principal del juego
│   ├── audio.rs          # Sistema de audio (música y efectos de sonido)
│   ├── caster.rs         # Algoritmo de raycasting y detección de colisiones
│   ├── enemy.rs          # Lógica de enemigos y IA
│   ├── framebuffer.rs    # Buffer de píxeles para renderizado
│   ├── input.rs          # Manejo de entrada del usuario
│   ├── intersect.rs      # Estructuras de intersección de rayos
│   ├── line.rs           # Utilidades para dibujar líneas
│   ├── maze.rs           # Carga y gestión de laberintos
│   ├── player.rs         # Jugador, movimiento y estado
│   ├── renderer.rs       # Renderizado 3D y sprites
│   ├── texture.rs        # Gestor de texturas e imágenes
│   └── ui.rs             # Interfaz de usuario (HUD, menú, pantallas)
│
├── assets/
│   ├── levels/
│   │   ├── maze.txt      # Nivel 1
│   │   └── maze2.txt     # Nivel 2
│   ├── textures/
│   │   ├── caja2.png     # Textura de pared normal
│   │   ├── caja3.png     # Textura de ladrillo
│   │   ├── caja4.png     # Textura especial
│   │   ├── caja5.png     # Textura de puerta
│   │   ├── tuveria1.png  # Textura de tubería 1
│   │   └── tuveria2.png  # Textura de tubería 2
│   ├── sprites/
│   │   ├── enemy.png     # Sprite de enemigo
│   │   ├── chest.png     # Sprite de cofre
│   │   ├── player_anim.png
│   │   └── work.png
│   └── audio/
│       ├── music.mp3     # Música de fondo
│       ├── collect.wav   # Sonido de recolección
│       ├── damage.wav    # Sonido de daño
│       └── footstep.wav  # Sonido de pasos
│
├── Cargo.toml            # Configuración del proyecto y dependencias
└── README.md             # Este archivo
```

## Tecnologías Utilizadas

- **Rust**: Lenguaje de programación principal
- **raylib-rs**: Biblioteca de gráficos y manejo de ventanas
- **rodio**: Biblioteca de audio

### Dependencias principales (Cargo.toml):
```bash
oml
[dependencies]
raylib = "5.0"
rodio = "0.19"
```

## Elementos del Juego

### Símbolos del Mapa:
- # - Pared normal (textura de caja)
- L - Pared de ladrillo
- $ - Puerta/Pared oscura
- T, P - Tuberías decorativas
- p - Posición inicial del jugador
- F - Enemigo
- C - Cofre
- E - Salida del nivel

### HUD (Heads-Up Display):
- **Barra de vida**: Esquina inferior izquierda (verde/amarillo/rojo)
- **Contador de cofres**: Muestra cofres recolectados/totales
- **FPS**: Esquina superior derecha
- **Nivel actual**: "Nivel X/2"
- **Batería de linterna**: Porcentaje de carga
- **Minimapa**: Esquina superior derecha
