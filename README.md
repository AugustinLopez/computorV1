# Synopsis

The *ComputorV1* program shall solve a polynomial equation of degree 2 or less.

![\Large P(X)=\sum_{i=0}^{n}\alpha_iX^n](https://latex.codecogs.com/gif.latex?P%28X%29%3D%5Csum_%7Bi%3D0%7D%5E%7Bn%7D%5Calpha_iX%5En)

It shall display the following information:
- The reduced form of the equation (*degree higher than 2 included*);
- The degree of the equation;
- Its solution(s), as well as the sign of the discriminant if relevant. Complex solutions included.



### Example
    > Argument: "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"
	> Reduced form: 4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0
    > Polynomial degree: 2
    > Discriminant is strictly positive, the two solutions are:
    > 0.905239
    > -0.475131

### Accepted equation strings:
Whitespaces are ignored. Two sign '+' and '-' cannot follow each other (*or themselves*).
- Basic: *"1 * X ^ 0 - 1 * X ^ 1 + 1.5 * X ^ 2 = 0 * X ^ 0"*
- Improved Degree 0/1 + coefficient 1: *"1 - X + 1.5 * X ^ 2 = 0"*
- No asterisk: *"1 - X + 1.5X ^ 2 = 0"*


### Options
- -h (*help*) : show the usage in the standard output.
- -l (*least common multiple*): try presenting decimal solutions as fraction.
- -d (*debug*) : print a step-by-step analysis of the program in the standard output.

# Ressources
- [Equation display for Markdown on Github](https://www.codecogs.com/latex/eqneditor.php)
- [Least Common Multiple](https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor)
- [Euclidian Algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm#Implementations) for the **Greated Common Divisor**