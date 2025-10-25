# Modelos 3D
Semestre 02, 2025



## Introducción


El formato Wavefront OBJ es un estándar de texto plano para describir geometría 3D.


Su objetivo principal es representar la malla poligonal de un objeto: una colección de vértices conectados en caras.


Es ampliamente usado porque es simple de leer, escribir e intercambiar entre programas.


Los `.obj` pueden complementarse con archivos `.mtl` que guardan materiales, colores y texturas.



## Conceptos fundamentales


Vértices (`v`): puntos en el espacio 3D definidos por coordenadas `(x, y, z)`. Son la base de la geometría.


Caras (`f`): superficies que unen vértices. Generalmente son triángulos o cuadriláteros.


Normales (`vn`): vectores perpendiculares a la superficie. Se usan para cálculos de iluminación.


Coordenadas de textura (`vt`): asignan posiciones 2D en una imagen (UV mapping) a cada vértice.


Materiales (`usemtl`): definen cómo se verá la superficie (color, textura, transparencia, reflejo).


Objetos y grupos (`o`, `g`): permiten organizar la geometría en partes.



## Malla poligonal

Conjunto de polígonos (caras) conectados por aristas y vértices.


La mayoría de los motores gráficos trabajan con triángulos porque siempre son planos y fáciles de procesar en la GPU.


La topología (cómo se conectan los vértices y caras) es tan importante como el número de polígonos.



## Bounding Box y Escala


Un bounding box es la caja mínima que encierra todo el modelo.


Se calcula con los valores mínimo y máximo de cada eje `(x, y, z)`.

Permite conocer:

  * Centro del modelo.
  * Escala relativa frente a otros objetos.
  * Normalización: proceso de centrar el modelo en el origen y escalarlo a una unidad estándar.


Esto es esencial para comparar modelos diferentes y usarlos en escenas sin inconsistencias.



## Usos


Interoperabilidad: casi cualquier software 3D entiende `.obj`.


Educación: es un formato legible que ayuda a aprender cómo se construye la geometría.


Motores 3D: necesitan los datos en forma de triángulos para renderizado eficiente.


Optimización: al analizar vértices, caras y bounding box, podemos decidir si un modelo es apto para tiempo real o necesita reducción.


Mapeado UV y materiales: permiten aplicar texturas realistas sobre geometría abstracta.



## Herramientas recomendadas


Blender: inspección, edición, exportación.
Meshlab: análisis, limpieza, simplificación.
Assimp: carga automática en proyectos.


Unity/Unreal: pruebas en tiempo real.
Python (trimesh/numpy): análisis por lotes.