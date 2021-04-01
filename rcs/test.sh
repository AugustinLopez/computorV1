prog='../computor'
j=0
i=1
step()
{
	printf "$i./ \"$1\"\n"
	${prog} "$1"
	printf "EXPECT \n$2\n"
	echo
	i=$(($i+1))
}

j=$(($j+1))
if [ "$1" = "" -o "$1" = "$j" ]; then
	echo "==== $j.SUBJECT ===="
	i=1
	step "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0" "0.905...\n-0.475..."
	step "5 * X^0 + 4 * X^1 = 4 * X^0" "-0.25"
	step "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0" "No Solution"
	step "5 + 4 * X + X^2= X^2" "-1.25/No Crash"
fi
j=$(($j+1))
if [ "$1" = "" -o "$1" = "$j" ]; then
	echo "==== $j.BASIC ERROR ==="
	printf "1./ No Argument\n"
	${prog}
	printf "EXPECT \nNothing\n"
	echo
	i=2
	step "" "Error"
	step "a" "Error"
	step "    a    " "Error"
	step "1a=0" "Error"
	step "a   =   0" "Error"
fi
j=$(($j+1))
if [ "$1" = "" -o "$1" = "$j" ]; then
	echo "==== $j.DEGREE 0===="
	i=1
	step "0 * X^0 = 0 * X^0" "True"
	step "-0 * X^0 = 0 * X^0" "True"
	step "0 * X^0 = -0 * X^0" "True"
	step "-0 * X^0 = -0 * X^0" "True"
	step "0.0 * X^0 = 0.00 * X^0" "True"
	step "-0 * X^0 = 0.0 * X^0" "True"
	step "0.00 * X^0 = -0 * X^0" "True"
	step "-0.00 * X^0 = -0.000 * X^0" "True"
	step "1 * X^0 = 1 * X^0" "True"
	step "-0.1 * X^0 = -0.1 * X^0" "True"
	step "42.42 * X^0 = 42.42 * X^0" "True"
	step "-21.001 * X^0 = -21.001 * X^0" "True"
	step "42.42 * X^0 = 0 * X^0" "False"
	step "1 * X^0 + 0 * X^1 + 0 * X^2 = 0 * X^0" "False"
	step "1.0 * X^0 + 0 * X^1 = 1 * X^0 + 0 * X^1 - 0 * X^2" "True"
fi
j=$(($j+1))
if [ "$1" = "" -o "$1" = "$j" ]; then
	echo "==== $j.DEGREE 1===="
	i=1
	step "1 * X^1 = 0 * X^0" "0"
	step "1.0 * X^1 = 1.0 * X^0" "1"
	step "3 * X^1 = 1.5 * X^0" "0.5"
	step "3 * X^1 = X^0" "0.333..."
	step "3 * X^1 = X^0 -1 * X^1 " "0.25"
	step "-3 * X^1 = X^0 - 1 * X^1" "-0.5"
	step "1000000 * X^1 = -X^0" "-1e-06/-0.000001"
	step "X^1 = 1000000 * X^0" "1000000"
	step "X^1 = 10000000000000 * X^0" "10000000000000"
fi
