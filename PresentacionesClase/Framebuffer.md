  Framebuffers y Bitmaps  

Framebuffers y Bitmaps
======================

Semestre 02, 2025

Utilidad
--------

*   Es la memoria intermedia entre el renderizado y la pantalla.
*   Permite construir la imagen completa antes de mostrarla.
*   Fundamental en el software rendering (CPU-only).

Definición
----------

Región de memoria reservada donde se almacena la imagen que está en proceso de construcción antes de enviarse a la pantalla.

Importancia
-----------

*   Permite representar gráficamente escenas complejas sin necesidad de dibujar directamente en la pantalla.
*   Es la base para técnicas modernas como doble buffer y triple buffer.
*   Optimiza el rendimiento al desacoplar el renderizado de la actualización de la pantalla.

Funcionamiento
--------------

El framebuffer actúa como un lienzo digital:

*   Cada píxel es representado por uno o varios bytes (color depth).
*   La GPU o CPU escribe directamente los valores de los píxeles.
*   El contenido del framebuffer se envía a la pantalla durante la fase de refresco.

### Flujo de Datos

*   Datos generados por shaders y pipelines gráficos son almacenados en el framebuffer.
*   Multiplexores seleccionan las señales RGB adecuadas para cada píxel.
*   DAC transforman los datos para pantallas.
*   Imagen renderizada es mostrada en la pantalla.

![](fb1.png)

*   El framebuffer es una cuadrícula de memoria que guarda los valores de color para cada píxel.
*   Los valores son índices que apuntan a una tabla de colores.
*   La tabla contiene los valores RGB completos.

![](fb2.png)

*   El Color Map alimenta cada canal de color (R, G, B) con valores digitales.
*   Los valores pasan por conversores D/A (Digital a Analógico) para generar señales para la pantalla.
*   Cada canal tiene su propio D/A.

Aplicaciones
------------

*   Doble buffering: evita parpadeos (tearing) al alternar entre dos framebuffers.
*   Post-procesado: efectos como bloom, blur o HDR se aplican sobre el framebuffer.

*   Captura de pantalla: se lee el contenido del framebuffer para guardar imágenes.

Bitmap
------

El formato más simple para representar imágenes digitales: una cuadrícula de píxeles almacenados en memoria.

### Definición

*   Una imagen rasterizada formada por una matriz de píxeles.
*   Cada píxel tiene un valor que representa su color.
*   Almacenada como datos binarios en la memoria o archivos.

![](bitmap.png)

### Profundidad de color

La cantidad de bits por píxel determina el número de colores posibles:

*   1 bit: 2 colores (blanco y negro).
*   4/8 bits: paletas de colores (16 o 256 colores).
*   16 bits: High Color (~65 mil colores).
*   24 bits: True Color (16.7 millones de colores).

_Mayor profundidad de color → mayor tamaño de archivo._

### Formato BMP

Bitmap (BMP) es un formato estándar para guardar imágenes rasterizadas:

*   Desarrollado por Microsoft e IBM en 1986.
*   Amplio soporte en sistemas operativos y programas.

#### Estructura del archivo:

*   **File Header:** identifica el archivo como BMP.
*   **Info Header:** contiene información de dimensiones, profundidad de color, compresión.
*   **Pixel Data:** la matriz de colores de los píxeles.

### Ventajas y desventajas

*   ✅ **Simplicidad:** fácil de leer y escribir.
*   ✅ **Sin pérdida de calidad:** no usa compresión.
*   ✅ **Ideal para pruebas y enseñanza.**
*   ❌ **Archivos grandes:** cada píxel ocupa espacio completo.
*   ❌ **No soporta transparencias ni capas.**

Reveal.initialize({ hash: true, slideNumber: false, progress: false });