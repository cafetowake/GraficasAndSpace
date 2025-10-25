# Primitive Assembly
Semestre 02, 2025



## Introducción


La etapa de Primitive Assembly ocurre después del Vertex Shader dentro del Render Pipeline.


Su función principal es conectar los vértices procesados por el Vertex Shader para formar primitivas geométricas:  
- Puntos  
- Líneas  
- Triángulos  


Estas primitivas son la unidad básica de dibujo que luego pasa al proceso de rasterización.



## ¿Por qué es necesaria?


El Vertex Shader trabaja vértice por vértice, de manera independiente.  


Pero la pantalla se dibuja con figuras completas, no con puntos aislados.


La etapa de Primitive Assembly toma esos vértices para realizar lo siguiente:


1. Los agrupa según el modo de dibujo (points, lines, triangles).  


2. Interpola atributos (color, normal, UV, etc.) entre ellos.  


3. Prepara las primitivas para las etapas de clipping y rasterización.  



## Tipos de primitivas


### 1. Puntos (`GL_POINTS`)

Cada vértice se convierte directamente en un píxel o punto en pantalla.  


### 2. Líneas

- `GL_LINES`: cada par de vértices forma una línea independiente.  
- `GL_LINE_STRIP`: forma una cadena de líneas conectadas.  
- `GL_LINE_LOOP`: igual que `LINE_STRIP`, pero conecta el último con el primero.  


### 3. Triángulos

- `GL_TRIANGLES`: cada grupo de 3 vértices forma un triángulo independiente.  
- `GL_TRIANGLE_STRIP`: cada nuevo vértice forma un triángulo con los dos anteriores.  
- `GL_TRIANGLE_FAN`: todos los triángulos comparten un vértice central.  


Los triángulos son los más comunes porque son planos, estables y fáciles de interpolar.  



## Proceso interno


### 1. Agrupación de vértices

El pipeline recibe un flujo ordenado de vértices del Vertex Shader.


Según el modo de dibujo, agrupa esos vértices en conjuntos:

- Modo puntos → grupos de 1  
- Modo líneas → grupos de 2  
- Modo triángulos → grupos de 3  


### 2. Interpolación de atributos

Cada vértice trae sus propios atributos:  
- Color  
- Normal  
- Coordenadas de textura  


Durante el ensamblado, estos valores se interpolan a lo largo de la línea o superficie de la primitiva.


Interpolación lineal entre dos vértices:

$$
A(t) = (1 - t) A_1 + t A_2,\quad 0 \le t \le 1
$$  


En triángulos, la interpolación se extiende a 2D mediante coordenadas baricéntricas:  

$$
A(P) = \lambda_1 A_1 + \lambda_2 A_2 + \lambda_3 A_3
$$  


donde  
$$
\lambda_1 + \lambda_2 + \lambda_3 = 1
$$  


### 3. Culling y orden de los vértices


Antes del clipping, se puede aplicar face culling:  
- Determina si una cara es visible o no, según el orden de los vértices (CW o CCW).  
- Si el triángulo está “de espaldas” a la cámara, puede descartarse.  


Esto se decide con el signo del determinante del triángulo en 2D:  

$$
\text{signo} = (x_2 - x_1)(y_3 - y_1) - (y_2 - y_1)(x_3 - x_1)
$$  


Si el signo es negativo, la cara mira hacia la cámara.  



## Interpolación de atributos


Para un punto interior P de un triángulo con vértices A, B, C:  


1. Se calculan los pesos baricéntricos:  
$$
\lambda_1 = \frac{A_{PBC}}{A_{ABC}}, \quad
\lambda_2 = \frac{A_{PCA}}{A_{ABC}}, \quad
\lambda_3 = \frac{A_{PAB}}{A_{ABC}}
$$  

donde $A_{XYZ}$ es el área del triángulo formado por los puntos X, Y, Z.  


2. Cualquier atributo (color, normal, UV) se interpola como:  
$$
A_P = \lambda_1 A_A + \lambda_2 A_B + \lambda_3 A_C
$$  

Esto garantiza una interpolación suave y continua en la superficie.  



## Importancia en el pipeline


- Define la forma final de lo que se dibuja.  


- Reduce el trabajo del rasterizador agrupando vértices.  


- Permite optimizaciones como vertex caching (reutilizar vértices).  


- Establece la base para la interpolación de fragmentos y la iluminación continua.  
