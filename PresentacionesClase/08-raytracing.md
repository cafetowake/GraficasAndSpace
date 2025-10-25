# Ray Tracing
Semestre 02, 2025



## Objetivo


Expandir el uso del concepto de un rayo para representar la realidad mejor en 2D y eventualmente en 3D.



## ¿Cómo vemos?


Los humanos percibimos la luz que llega a nuestros ojos luego de interactuar con múltiples objetos en el camino.


![fov0](../assets/img/see.png)


La luz blanca contiene todas las longitudes de onda.


![fov0](../assets/img/pink_floyd.png)


Los objetos absorben y reflejan diferentes longitudes de onda.


El color que vemos es el resultado de la luz reflejada que llega a nuestros ojos.



## Rayos 3D


Queremos replicar el comportamiento de la luz en un entorno 3D


- Los rayos salen de la fuente de luz.
- Impactan en objetos y se reflejan en todas direcciones.
- Una parte de esos rayos llega a nuestros ojos.


![fov0](../assets/img/rt-1.png)


**Problema:**  

El número de rayos posibles es prácticamente infinito, lo que hace imposible simularlos todos.

![fov0](../assets/img/rt-2.png)


**Solución:**

En lugar de lanzar rayos desde la luz hacia los objetos, lanzamos rayos desde la cámara (nuestro punto de vista) hacia la escena.

![fov0](../assets/img/rt-3.gif)


- Cada rayo recorre el espacio hasta encontrar un objeto o llegar al infinito.
- En cada intersección, calculamos el color resultante.
- Este proceso se repite **un rayo por cada píxel** de la imagen.



## Rayos primarios y secundarios


**Rayos primarios:** salen desde la cámara hacia la escena.


**Rayos de sombra:** se lanzan desde el punto de intersección hacia las luces para determinar iluminación.


**Rayos reflejados/refractados:** simulan materiales como espejos, vidrio o agua.


![fov0](../assets/img/rt-4.gif)



## Construyendo la imagen


La combinación de todos los rayos genera la imagen final.


![fov0](../assets/img/rt-5.gif)


Es una proyección de alta definición sin transformaciones geométricas.


Conceptualmente, es un **cíclo doble** que recorre todos los píxeles.



## Normalización de direcciones


Para lanzar un rayo necesitamos definir su dirección en un sistema de coordenadas que sea consistente y fácil de manejar matemáticamente.


Esto se hace en un espacio de coordenadas normalizado, donde todos los valores se expresan en un rango y escala conocida.


Las pantallas usan coordenadas **de píxel** con un origen en la esquina (0,0) y valores positivos hacia la derecha (x) y hacia abajo (y).


Si trabajáramos directamente en ese espacio, los cálculos de intersecciones y proyecciones dependerían del tamaño exacto de la ventana lo que no es reutilizable.


Al mapear los píxeles a un rango [-1, 1] en X e Y, creamos un sistema independiente de la resolución.

![fov0](../assets/img/norm.png)


Podemos cambiar la resolución sin alterar la lógica matemática del trazado de rayos



## Aspect Ratio


La mayoría de las pantallas no son cuadradas, así que el eje X debe escalarse para que los rayos se proyecten con la geometría correcta.


Si no se ajusta el aspect ratio, un círculo podría verse como una elipse estirada horizontalmente o verticalmente.


$ratio = \frac{ancho}{alto}$


Multiplicamos la coordenada X por este factor antes de normalizar.


Evita que la imagen se estire o comprima horizontal o verticalmente.

![fov0](../assets/img/aspect.png)


## Logitud Unitaria


Aplicamos normalización para que todos los rayos tengan longitud unitaria.


Una dirección en 3D se representa como un vector, si ese vector tiene una longitud arbitraria, la magnitud podría distorsionar el resultado.


Normalizar significa dividir el vector por su propia longitud:

$\vec{d}_{norm} = \frac{\vec{d}}{\|\vec{d}\|}$



## Luz difusa y especular


En gráficos por computadora, la interacción entre la luz y la superficie de un objeto se modela usando diferentes componentes de iluminación. Las dos más comunes son difusa y especular.


### Luz Difusa


Representa la forma en que la luz se dispersa uniformemente en todas las direcciones después de chocar sobre una superficie rugosa o mate.


La intensidad de la luz difusa depende únicamente del ángulo entre la luz y la normal de la superficie, no de la posición del observador.


![fov0](../assets/img/difuse.png)


### Luz Especular


Representa la luz que se refleja de forma más concentrada en una dirección específica, típica de superficies lisas o pulidas.


La intensidad especular sí depende de la posición del observador.


Es un reflejo, o brillo.


La combinación de ambos modelos genera realismo.



## Objetos y materiales


En ray tracing, los objetos y materiales definen qué forma tiene el mundo y cómo responde a la luz.


Para calcular intersecciones necesitamos 2 factores:

- **Geometría:** Esferas, planos, triángulos.
- **Materiales:** Definen cómo se comporta la luz (difusa, especular, translúcida).

![fov0](../assets/img/materials.png)



## Z-buffer


El Z-Buffer es una estructura de datos usada en renderizado 3D para determinar qué objetos o partes de objetos son visibles en la escena.


Almacena información de profundidad (distancia desde la cámara) para cada píxel.


Cuando dos objetos se superponen en la imagen, el que tiene menor valor de profundidad (más cercano) es el que se muestra.


### Implementación


1. Inicialización

Antes de renderizar, el Z-Buffer se llena con un valor máximo de profundidad (infinito).


2. Render

- Calcular valor de profundidad (d a cámara).
- Comprara valor almacenado del Z-buffer.
- Más cerca se cambia el color y profundidad.
- Más lejos se descarta.


3. Resultado

Solo se dibujan los pixeles visibles lo que da una ilusión óptica de profundidad.


### Limitaciones


Problemas de z-fighting cuando los objetos están muy cerca.


Alto uso de memoria.


Caro, ya que compara cada pixel.