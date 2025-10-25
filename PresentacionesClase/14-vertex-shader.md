# Vertex Shader
Semestre 02, 2025



## Introducción


El **Vertex Shader** es la **primera etapa programable** de la GPU dentro del *Render Pipeline*.


- Procesa **cada vértice** de los modelos 3D.
- Aplica **transformaciones geométricas** para llevarlos de su espacio local → pantalla.
- Genera atributos que serán usados en etapas posteriores (normales, UV, colores).


Es la etapa donde conectamos la **matemática lineal** con el **código gráfico**.



## Usos


- Los modelos 3D son listas de **vértices** (puntos en el espacio).


- Un vértice por sí solo no tiene sentido en la pantalla.


- El Vertex Shader transforma esos puntos en una representación que la GPU puede proyectar y rasterizar.


**Sin vertex shader:** el modelo se quedaría en coordenadas locales.


**Con vertex shader:** podemos ver el modelo correctamente en la pantalla, en la posición, escala y orientación deseada.



## Flujo general


1. Recibe **atributos por vértice**: posición, normal, color, coordenadas de textura.


2. Aplica transformaciones con **matrices 4x4**.


3. Pasa los atributos necesarios al siguiente paso (rasterización).


4. Produce como salida obligatoria `gl_Position`, la posición del vértice en **clip space**.



## Repaso matemático


Un vértice pasa por una cadena de **espacios de coordenadas**:


1. **Espacio de Objeto (Model space)**: coordenadas relativas al modelo.


2. **Espacio de Mundo (World space)**: el objeto colocado en la escena.


3. **Espacio de Vista (View space)**: escena vista desde la cámara.


4. **Clip Space**: después de la proyección.


5. **NDC (Normalized Device Coordinates)**: división por w, rango [-1, 1].


6. **Viewport**: mapeo final a píxeles.


El Vertex Shader se encarga de los primeros 4 pasos.



## Matriz de Modelo (M)


- Aplica **traslaciones, rotaciones y escalas** a los vértices.


- Coloca el objeto en su posición dentro de la escena.


Ejemplo (traslación en x = 5):

$$
M = 
\\begin{bmatrix}
1 & 0 & 0 & 5 \\\\
0 & 1 & 0 & 0 \\\\
0 & 0 & 1 & 0 \\\\
0 & 0 & 0 & 1
\\end{bmatrix}
$$


Si el vértice es (x, y, z, 1):


$$
M \\cdot P = (x+5, y, z, 1)
$$



## Matriz de Vista (V)


- Simula la **cámara**.
- Coloca la cámara en el origen mirando hacia -Z.
- Se construye con una combinación de rotación + traslación inversa de la cámara.

Ejemplo: la cámara en (0, 0, 5) mirando al origen.

$$
V = T^{-1}_{cam} R^{-1}_{cam}
$$


En la práctica se calcula con funciones como `lookAt(eye, center, up)`.



## Matriz de Proyección (P)


- Aplica la proyección (perspectiva u ortográfica).
- Define el **frustum** (volumen visible).


### Proyección perspectiva:

$$
P = 
\\begin{bmatrix}
\\tfrac{1}{\\tan(\\theta/2) \\cdot a} & 0 & 0 & 0 \\\\
0 & \\tfrac{1}{\\tan(\\theta/2)} & 0 & 0 \\\\
0 & 0 & \\tfrac{f+n}{n-f} & \\tfrac{2fn}{n-f} \\\\
0 & 0 & -1 & 0
\\end{bmatrix}
$$

- θ = FOV (Field of View).
- a = aspect ratio.
- n = near plane.
- f = far plane.



## Transformación completa (MVP)


$$
P_{clip} = P \\cdot V \\cdot M \\cdot P_{objeto}
$$

- `M` ubica el modelo en la escena.
- `V` aplica la cámara.
- `P` proyecta en perspectiva.


El resultado es un vértice en **clip space**.



## Coordenadas homogéneas


- Los puntos se representan como `(x, y, z, 1)`.


- Las direcciones (ej. normales) como `(x, y, z, 0)`.


La cuarta coordenada **w**:
- Permite aplicar traslaciones con matrices.
- Se usa luego para la **perspective divide**:


$$
(x_{ndc}, y_{ndc}, z_{ndc}) = (x/w, y/w, z/w)
$$



## Transformación de Normales


- Las normales se transforman distinto a las posiciones.


- Se usa la **matriz inversa transpuesta** del modelo:


$$
n' = (M^{-1})^T n
$$


Esto evita deformaciones en la iluminación cuando hay escalas no uniformes.



## Conexión con .obj


- Los modelos `.obj` son listas de vértices (`v`), normales (`vn`) y caras (`f`).


- El Vertex Shader toma esos vértices y los transforma.


- Ejemplo de línea en un `.obj`:

```obj
v 1.0 0.0 0.0
vn 0.0 0.0 1.0
f 1//1 2//1 3//1
```


- El shader recibe esa información como atributos y la transforma a clip space.



## Próximos pasos


- Entender cómo los vértices transformados se agrupan en primitivas.


- Estudiar el **Primitive Assembly** y el clipping.


- Introducir la etapa de rasterización.


Con el Vertex Shader dominado, ya tenemos la base del pipeline programable.
