# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: aulopez <aulopez@student.42.fr>            +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2021/01/21 11:54:10 by aulopez           #+#    #+#              #
#    Updated: 2021/01/21 14:11:42 by aulopez          ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME=computor
PATH_SRC=./src/
SOURCE=main option parsing parsing/expected
SRC=$(SOURCE:%=$(PATH_SRC)%.rs)

all: $(NAME)

$(NAME): $(SRC)
	cargo build --release
	cp ./target/release/$(NAME) .

clean:
	cargo clean

fclean: clean
	rm -f $(NAME)

re: fclean all

install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

uninstall:
	rustup self uninstall

.PHONY: all clean fclean re
