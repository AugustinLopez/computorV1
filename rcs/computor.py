from string import whitespace
from sys import argv
from re import split

class Polynomial:
    """
    """
    debug=False
    lcm=False
    def __init__(self, coefficients=None):
        """
        Argument accepted: list or dictionary.
        - First element of the list is X^0 coefficient, second is X^1, etc...
        - Key is the power, Value is the coefficient.
        """
        if isinstance(coefficients, list):
            self.__init_from_list(coefficients)
            msg="List"
        elif isinstance(coefficients, dict):
            self.__init_from_dict(coefficients)
            msg="Dictionary"
        elif coefficients==None:
            self.__init_from_dict({0:0})
            msg=None
        else:
            raise TypeError("Unsupported type provided to __init__")
        if self.debug:
            print("Polynomial initialized from '", msg, "'",sep="")
            print(self)


    def __init_from_list(self, coefficients):
        """
        Initialize Polynomial from a list of coefficient.
        First element of the list is X^0 coefficient, second is X^1, etc...
        """
        self.degree=len(coefficients)-1
        self.coef= {i:self.degree for i in coefficients}
        for c in self.coef:
            if not c.isnumeric():
                raise TypeError("List contains non-numeric values")


    def __init_from_dict(self, coefficients, degree=None):
        """
        Initialize Polynomial from a dictionary.
        Key is the power, Value is the coefficient.
        """
        for key, value in coefficients.items():
            if not (isinstance(key, int) and key >=0):
                raise TypeError("Dictionary keys must be positive integers")
            elif not (isinstance(value, int) or isinstance(value, float)):
                raise TypeError("Dictionary values must be number")
        self.coef={}
        self.degree=0
        for key, value in sorted(coefficients.items()):
            if key > self.degree and value != 0:
                self.degree=key
            self.coef[key]=value


    @classmethod
    def __debug_index(cls,group,i):
        if cls.debug==True:
            print("    Index: ",i," - Element: \"",group[i],"\"", sep='')


    @classmethod
    def __from_equation_feed_dict(cls, sign, mul, degree):
        """
        Feed a dictionary from the equation loop.
        """
        if degree==None and mul==None:
            return None, None
        if degree==None:
            degree=0
        old=cls.coef.get(degree,0)
        if mul==None:
            mul=1
        new=old+sign*mul
        if new==int(new):
            cls.coef[degree]=int(new)
        else:
            cls.coef[degree]=new
        if cls.debug==True:
            if sign*mul==int(sign*mul):
                print("  Entry: {",degree,": ",int(sign*mul),"}",sep="")
            else:
                print("  Entry: {",degree,": ",sign*mul,"}",sep="")
            print("  Dictionary state:",cls.coef)
        return None, None


    @classmethod
    def __from_equation_number(cls, group, i):
        """
        called if group start with a digit.
        - Group must be a number.
        - if number followed by "*", "*" must be followed by "X".
        """
        elem=group[i]
        index=elem.find("x")
        end=None
        if index==-1:
            index=elem.find("X")
        if index!=-1:
            end=elem[index:]
            if end!="X" and end!="x":
                raise SyntaxError("Invalid character in equation")
            elem=elem[0:index]
        try:
            if elem.find(".")!=-1:
                num=float(elem)
            else:
                num=int(elem)
        except ValueError:
            raise SyntaxError("Invalid character in equation")
        if i<len(group)-1 and group[i+1]=="*":
            cls.__debug_index(group,i+1)
            if end != None:
                raise SyntaxError("Invalid use of '*' sign")
            if i < len(group)-2 and (group[i+2]!="X" and group[i+2]!="x"):
                raise SyntaxError("Invalid use of '*' sign")
            return i+1,num
        return i,num


    @classmethod
    def __from_equation_X(cls, group, i):
        """
        called if group is "X"
        - If next group is not "^", then degree is 1.
        - Power must be a positive int
        """
        if i==len(group)-1 or group[i+1]!="^":
            return i, 1
        cls.__debug_index(group,i+1)
        if i+2>=len(group):
            raise SyntaxError("Invalid Last character '^'")
        cls.__debug_index(group,i+2)
        if not group[i+2].isnumeric() or group[i+2].find(".")!=-1:
            raise SyntaxError("Invalid Power")
        degree=int(group[i+2])
        if degree < 0:
            raise SyntaxError("Negative Power not supported")
        return i+2, degree


    @classmethod
    def __from_equation_sign(cls, group, i, left_hand):
        """
        The the minus sign can follow the '=' sign and start the string.
        the plus sign cannot.
        '+' and '-' cannot follow each other.
        """
        if group[i]=="-":
            if left_hand > 0:
                return 1
            else:
                return -1
        elif group[i]=="+":
            if i==0 or (left_hand > 0 and i==left_hand+1):
                raise SyntaxError("Bad use of '+' sign")
            if left_hand > 0:
                return -1
            else:
                return 1


    @classmethod
    def __from_equation_loop(cls, group):
        """
        Parsing loop:
        - left_hand tell us if we are right or left of the '=' sign, reversing
        the value of the sign.
        - dbl_sign check if two sign follow one another.
        - degree/mul is the couple key/value
        """
        cls.coef={}
        left_hand=0
        dbl_sign=False
        degree=None
        sign=1
        mul=None
        i=0
        while i<len(group):
            if group[i]=="":
                i=i+1
                pass
            if group[i][0].isnumeric():
                cls.__debug_index(group,i)
                if group[i].find("x")!=-1 or group[i].find("X")!=-1:
                    xx=True
                else:
                    xx=False
                i, mul=cls.__from_equation_number(group, i)
                if xx==True:
                    i, degree=cls.__from_equation_X(group, i)
                dbl_sign=False
            elif group[i]=="X" or group[i]=="x":
                cls.__debug_index(group,i)
                i, degree=cls.__from_equation_X(group, i)
                dbl_sign=False
            elif group[i]=="-" or group[i]=="+":
                if dbl_sign==True:
                    cls.__debug_index(group,i)
                    raise SyntaxError("Forbidden double sign")
                dbl_sign=True
                degree, mul=cls.__from_equation_feed_dict(sign,mul,degree)
                cls.__debug_index(group,i)
                sign=cls.__from_equation_sign(group, i, left_hand)
            elif group[i]=="=":
                if i==0 or left_hand > 0:
                    raise SyntaxError("Bad use of '=' sign")
                dbl_sign=False
                degree, mul=cls.__from_equation_feed_dict(sign,mul,degree)
                cls.__debug_index(group,i)
                if cls.debug==True:
                    print("  Sign '=' found: future sign reversed")
                left_hand=i
                sign=-1
            else:
                cls.__debug_index(group,i)
                raise SyntaxError("Invalid Equation member")
            i=i+1
        if left_hand==0:
            raise SyntaxError("No '=' sign found")
        else:
            degree, mul=cls.__from_equation_feed_dict(sign,mul,degree)
        if cls.debug==True:
            print("    Index: OVER")
        return cls(cls.coef)


    @classmethod
    def from_equation(cls, equation=None):
        """
        Initialize Polynomial from an equation represented by a string.
        Whitespace are ignored.
        Minimum length is 3, the form "a=b"
        """
        if equation==None:
            raise TypeError("No argument provided")
        if not isinstance(equation, str):
            raise TypeError("Argument must be a string")
        no_white=equation.translate({ord(c): None for c in whitespace})
        if len(no_white)<3:
            raise SyntaxError("String argument is too short")
        group=split('([\-\+\=\*\^])', no_white)
        if group[0]=="":
            group.pop(0)
        if cls.debug==True:
            print("Equation split:",group)
        if not (group[0][0].isnumeric() or group[0]=="-" or group[0]=="X" \
        or group[0]=="x"):
            raise SyntaxError("Illegal first character")
        return cls.__from_equation_loop(group)


    def determinant(self):
        """
        Calculate and return determinant if degree == 2
        """
        if self.degree!=2:
            self.determinant=None
            return None
        a=self.coef[2]
        try:
            b=self.coef[1]
        except:
            self.coef[1] = 0.0
            b = 0.0
        try:
            c=self.coef[0]
        except:
            self.coef[0] = 0.0
            c = 0.0
        self.determinant=b**2-4*a*c
        return self.determinant


    def __str__(self):
        """
        Return a string containing the reduced form and the degree of
        the polynomial equation.
        """
        ret="Reduced form: "
        first=True
        for key, value in sorted(self.coef.items()):
            if float(value)==int(value):
                value=int(value)
            if value!=0:
                if first==True:
                    first=False
                    if value ==-1 and key!=0:
                        ret=ret+"-"
                    elif not (value==1 and key!=0):
                        ret=ret + str(value)
                else:
                    if value==1:
                        ret=ret+" + "
                    elif value==-1:
                        ret=ret+" - "
                    elif value>0:
                        ret=ret + " + " + str(value)
                    else:
                        ret=ret + " - " + str(-value)
                if value!=1 and value!=-1 and key!=0:
                    ret=ret+" * "
                if key==1:
                    ret=ret+"X"
                elif key>0:
                    ret=ret+"X^"+str(key)
        if first==True:
            ret=ret + "0 = 0"
        else:
            ret=ret + " = 0"
        ret=ret+"\nPolynomial degree: "+str(self.degree)
        return ret


    def __gcd(self,a, b):
        while (b != 0):
            t=b
            b=a%b
            a=t
        return a


    def __lcm(self,a,b):
        if a < 0:
            a=-a
        if b < 0:
            b=-b
        if a == 0 or b == 0:
            return 0
        return (a*b)/self.__gcd(a,b)

    def __sol(self,a,b):
        sol=-a/b
        if self.lcm==False:
            if sol==int(sol):
                return str(int(sol))
            else:
                return str(sol)
        i=10
        ak=a*10
        bk=b*10
        while (a!=int(a) or b!=int(b)) and ak/i==a and bk/i==b:
            a=ak
            b=bk
            ak=ak*10
            bk=bk*10
        if a==int(a) and b==int(b) and sol != int(sol):
            lcm=self.__lcm(a, b)
            den=int(lcm/a)
            nom=int(-lcm/b)
            if den < 0:
                den=-den
                nom=-nom
            return str(nom) + "/" + str(den)
        else:
            if sol==int(sol):
                return str(int(sol))
            else:
                return str(sol)

    def solution(self):
        if self.degree>2:
            print("The polynomial degree is strictly greater than 2", \
                  "I can't solve.")
        elif self.degree==0:
            print("The equation is:")
            if self.coef.get(0) is None:
                print(True)
            else:
                print((self.coef[0]==0))
        elif self.degree==1:
            print("The solution is:")
            if self.coef.get(0) is None:
                print(self.__sol(0.0,self.coef[1]))
            else:
                print(self.__sol(self.coef[0],self.coef[1]))
        elif self.degree==2:
            deter=self.determinant()
            if deter<0:
                print("Discriminant is strictly negative", \
                      ", the two solutions are:", sep="")
                imag=(-deter)**0.5/(2*self.coef[2])
                if imag==int(imag):
                    imag=int(imag)
                real=-self.coef[1]/(2*self.coef[2])
                if real==int(real):
                    real=int(real)
                if imag > 0:
                    print(str(real) + " + " + str(imag) + " * i ")
                    print(str(real) + " - " + str(imag) + " * i ")
                else:
                    print(str(real) + " + " + str(-imag) + " * i ")
                    print(str(real) + " - " + str(-imag) + " * i ")
            elif deter==0:
                print("Discriminant is zero, the solution is:")
                print(self.__sol(self.coef[1],2*self.coef[2]))
            elif deter>0:
                print("Discriminant is strictly positive", \
                      ", the two solutions are:", sep="")
                print(self.__sol(self.coef[1]+deter**0.5,2*self.coef[2]))
                print(self.__sol(self.coef[1]-deter**0.5,2*self.coef[2]))

def usage():
    print()
    print("Usage: ./computor [-d/-l/-h] \"argument\"", sep="")
    print("\t-d (debug)\t print step-by-step analysis")
    print("\t-l (lcm)  \t try printing irreductible fractions")
    print("\t-h (help) \t print usage")


if __name__ == "__main__":
    i=1
    helper=False
    if len(argv) == 1:
        usage()
        helper=True
    if len(argv) >= 2:
        while argv[i]=="-d" or argv[i]=="-l":
            if argv[i]=="-d":
                Polynomial.debug=True
            else:
                Polynomial.lcm=True
            i=i+1
        if argv[i]=="-h":
            usage()
            helper=True
    if len(argv) > i and helper==False:
        try:
            test=Polynomial.from_equation(argv[i])
            if Polynomial.debug==False:
                print(test)
            test.solution()
        except SyntaxError as e:
            print("Error:",e)
