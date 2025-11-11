# Raycaster

Raycaster esta desarrollado en Rust como proyecto del curso de Gráficas por Computadora. Un juego de exploración con perspectiva first-person donde debes navegar por laberintos evitando obstáculos peligrosos (lava) para llegar a la meta.


![Juego](assets/gif/proyect1.gif)


Si no logra visualizar el .gif en el apartado de arriba, por favor ingrese al siguiente enlace para ver el video: 
https://www.canva.com/design/DAG4bDh6sXA/Or-gI1wo2eDjqyOd17-YtA/edit?utm_content=DAG4bDh6sXA&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton

## De qué trata el juego

Eres un explorador atrapado en una serie de mazmorras llenas de peligros. Tu objetivo es simple: **navegar por el laberinto y llegar al portal de salida** sin perder todas tus vidas.

### Obstáculos
- Lava: Bloques naranjas brillantes que te quitan vida al tocarlos
- Paredes: No puedes atravesarlas, debes encontrar el camino correcto

### Cómo ganar
Encuentra y alcanza el **portal cristalino** (bloque blanco/morado brillante) para completar el nivel. Si pierdes todas tus vidas, respawneas en el inicio del nivel.

### Niveles disponibles
1. **Nivel 1 - Easy**: Tutorial básico con pocas trampas
2. **Nivel 2 - Complicado**: Laberinto más complejo con más áreas de lava

## Controles

### Movimiento
- **W** - Avanzar
- **S** - Retroceder  
- **A** - Moverse a la izquierda
- **D** - Moverse a la derecha
- **Mouse** - Rotar la cámara (horizontal)

### Menú y Sistema
- **1 / 2** - Seleccionar nivel en el menú principal
- **Enter** - Comenzar juego
- **ESC** - Pausar / Volver al menú
- **Tab** - Liberar mouse (útil para salir de fullscreen)
- **Space** - Continuar después de ganar

## Cómo ejecutar el programa

### Prerrequisitos
Necesitas tener instalado Rust. Si no lo tienes:
```bash
# Windows / Linux / Mac
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Instalación y ejecución

```bash
# 1. Clonar el repositorio
git clone https://github.com/miafuentes30/Proyecto-1-Raycasting.git

# 2. Compilar y ejecutar (modo release para mejor rendimiento)
cargo run --release

# Opcional: solo compilar
cargo build --release
# El ejecutable quedará en: target/release/raycaster-project
```

### Si tienes problemas con audio
El juego funciona sin archivos de audio. Si ves errores relacionados con sonido, simplemente ignóralos - el juego seguirá funcionando normalmente.


## Estructura del proyecto

```
raycaster-project/
├── src/
│   ├── main.rs         # Loop principal y eventos
│   ├── raycaster.rs    # Motor de raycasting
│   ├── player.rs       # Lógica del jugador
│   ├── map.rs          # Definición de niveles
│   ├── draw.rs         # UI y efectos visuales
│   └── util.rs         # Utilidades
├── assets/
│   └── audio/          # Archivos de audio (opcional)
│   └── gif/          # Archivos de audio (opcional)
├── Cargo.toml
└── README.md
```

## Tipos de bloques/texturas

El juego usa texturas procedurales generadas por código:

| Color/Textura | Tipo | Efecto |
|---------------|------|--------|
| Rojo oscuro | Ladrillos | Bloquea paso |
| Verde | Piedra con musgo | Bloquea paso |
| Azul | Azulejos | Bloquea paso |
| Dorado | Oro metálico | Bloquea paso |
| Naranja/Rojo | Lava | ¡Causa daño! |
| Cian/Morado | Portal | ¡Meta del nivel! |


## Tecnologías usadas

- **Lenguaje**: Rust 
- **Renderizado**: pixels (frame buffer directo)
- **Ventanas**: winit
- **Audio**: rodio
