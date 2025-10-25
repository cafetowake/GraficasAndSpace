## Transformaciones Geométricas
Semestre 02, 2025



## Objetivo

* Comprender las transformaciones geométricas básicas: **traslación**, **escalado** y **rotación**.
* Aplicar estas transformaciones usando **matrices**.
* Entender su implementación en el render loop de una aplicación gráfica en tiempo real.



## Transformaciones geométricas


Una transformación geométrica permite **modificar la posición, tamaño u orientación** de un objeto en el espacio.


En gráficos por computadora, representamos los objetos como **vértices**, y transformamos esos vértices con **matrices**.


El uso de matrices permite representar todas las transformaciones lineales (rotación, escalado, reflexión) y transformaciones afines (traslación) de forma uniforme mediante multiplicación matricial.



## Transformaciones básicas


### Traslación

Desplaza un objeto en el espacio. No cambia su tamaño ni su orientación.


<p>
    $$ 
    \begin{bmatrix}
    x' \\
    y' \\
    1
    \end{bmatrix}
    =
    \begin{bmatrix}
    1 & 0 & t_x \\
    0 & 1 & t_y \\
    0 & 0 & 1
    \end{bmatrix}
    \begin{bmatrix}
    x \\
    y \\
    1
    \end{bmatrix}
    $$
</p>

Donde $t_x$ y $t_y$ son las componentes del vector de traslación.


### Escalado


Modifica el tamaño del objeto (ampliar o reducir).


<p>
    $$ 
    \begin{bmatrix}
    x' \\
    y' \\
    1
    \end{bmatrix}
    =
    \begin{bmatrix}
    s_x & 0 & 0 \\
    0 & s_y & 0 \\
    0 & 0 & 1
    \end{bmatrix}
    \begin{bmatrix}
    x \\
    y \\
    1
    \end{bmatrix}
    $$
</p>

Donde $s_x$ y $s_y$ son los factores de escala en cada eje.


### Rotación


Rota un objeto alrededor del origen (0,0), o de un punto específico.


<p>
    $$ 
    \begin{bmatrix}
    x' \\
    y' \\
    1
    \end{bmatrix}
    =
    \begin{bmatrix}
    \cos \theta & -\sin \theta & 0 \\
    \sin \theta & \cos \theta & 0 \\
    0 & 0 & 1
    \end{bmatrix}
    \begin{bmatrix}
    x \\
    y \\
    1
    \end{bmatrix}
    $$
  </p>

Para rotar alrededor de un centro, primero se traslada el objeto a (0,0), se rota, y luego se traslada de nuevo a su posición original.



## Composición de transformaciones


Las transformaciones se pueden **combinar** multiplicando las matrices en el orden adecuado. La clave está en que el orden **importa**. Por ejemplo:


* Primero trasladas al origen.
* Luego escalas o rotas.
* Finalmente, trasladas de vuelta a su posición original.


$$
T_{final} = T_{back} \cdot S \cdot R \cdot T_{to\_origin}
$$


| Sigla        | Significado                                                         |
| ------------ | ------------------------------------------------------------------- |
| `T_toorigin` | **Translation to origin** – mueve el objeto para centrarlo en (0,0) |
| `R`          | **Rotation** – aplica rotación alrededor del origen                 |
| `S`          | **Scale** – aplica la escala desde el origen                        |
| `T_back`     | **Translation back** – mueve el objeto de regreso a su posición     |
| `T_final`    | **Final transformation matrix** – resultado de aplicar todo         |




## Matrices 3x3


Se usan **coordenadas homogéneas** para representar $(x, y)$ como $(x, y, 1)$.


Esto permite incluir la traslación como parte de la multiplicación matricial.


Las matrices 3x3 permiten combinar rotación, escalado y traslación en una sola operación eficiente.


En 3D se usan matrices 4x4 para incluir transformaciones en 3 dimensiones.