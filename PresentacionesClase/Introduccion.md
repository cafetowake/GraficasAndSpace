  Introducción a Gráficas  

Introducción a Gráficas
=======================

Semestre 02, 2025

Definición
----------

*   Generar imágenes mediante computadoras.

Casos de uso
------------

*   Videojuegos, simulaciones, cine (CGI), AR/VR.
*   Entretenimiento, medicina, ingeniería, ciencia.

![](gta.jpg)

![](gta2.jpg)

![](lara.jpg)

Historia
--------

*   **Años 50–60:** Los primeros gráficos, osciloscopios y plotters.
*   **Años 70:** Nace la rasterización, Algoritmo de Bresenham, Pong (1972).

*   **Años 80:** Se desarrolla OpenGL (1982), Gráficos vectoriales y sombreados más complejos en cine y videojuegos.
*   **Años 90:** Aparece DirectX (1995), GPUs. _Jurassic Park_ muestran CGI avanzado.
*   **2000–2010:** Shaders, GPU núcleo de gráficos 3D en tiempo real (NVIDIA, ATI).

*   **Presente:** Ray tracing en tiempo real (NVIDIA RTX), Unreal Engine 5, Iluminación global dinámica, Realidad virtual y aumentada.

Conceptos clave
---------------

*   Pixel, resolución, modelos de color (RGB, CMYK).
*   Sistemas de coordenadas: mundo, cámara, pantalla.

![](Lakitu.jpg)

Tipos de gráficos
-----------------

*   Raster: imágenes como matrices de píxeles (BMP, PNG).
*   Vectoriales: primitivas geométricas.
*   2D vs. 3D: planos vs. espacio tridimensional.

Pipeline gráfico
----------------

Proceso mediante el cual se transforma un mundo 3D en una imagen 2D lista para mostrar en pantalla.

### Modelado

Definición de los objetos y su geometría en un espacio 3D. Puntos, líneas, polígonos.

### Transformación

Aplicación de operaciones matemáticas para mover, rotar o escalar los objetos en el espacio 3D.

Usa matrices y álgebra lineal para transformar las coordenadas de los vértices.

### Iluminación

Cálculo de cómo las fuentes de luz afectan las superficies de los objetos.

### Proyección

Conversión de la escena 3D a un espacio 2D aplicando una cámara virtual.

### Rasterización

Proceso de convertir las primitivas 2D (triángulos, líneas) en píxeles en el framebuffer.

Incluye el Z-buffer para determinar qué píxeles son visibles y ocultar superficies traseras.

Píxel
-----

Un **píxel** es la unidad más pequeña de una imagen digital.

Es un punto en una cuadrícula 2D que representa un color.

Un píxel (picture element) almacena información de color y brillo.

En una pantalla, millones de píxeles forman las imágenes que vemos.

### Resolución

La cantidad de píxeles en la pantalla define la **resolución**.

Ejemplo: 1920x1080 significa 1920 columnas y 1080 filas de píxeles.

A mayor resolución, mayor detalle.

### Profundidad de color

Cada píxel puede representar una gama de colores dependiendo de su profundidad de bits.

*   1 bit: Monocromo​
*   4 bits: Palettized (color map)​
*   8 bits: Palettized (color map)​
*   16 bits: High color​
*   24 bits: True color

### Modelo de color RGB

Cada píxel está compuesto por tres valores de intensidad:

*   **R**: rojo
*   **G**: verde
*   **B**: azul

La combinación de estos tres canales define el color final.

![](pixels.png)

El futuro
---------

*   Realidad virtual y aumentada.
*   Inteligencia artificial en gráficos.
*   Metaverso y simulación de mundos completos.

Recomendaciones
---------------

Repasar álgebra lineal.

![](right.jpg)

Reveal.initialize({ hash: true, });