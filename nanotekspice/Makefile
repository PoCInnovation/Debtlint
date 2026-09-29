##
## EPITECH PROJECT, 2026
## Makefile
## File description:
## Makefile
##


CXX 		= 	clang++

SRC			=	src/main.cpp			\
				src/Gate.cpp			\
				src/Circuit.cpp			\
				src/Shell.cpp			\

OBJ			=	$(SRC:.cpp=.o)

NAME		=	nanotekspice

CXXFLAGS	=	-Wall -Wextra -std=c++20

CPPFLAGS	= 	-iquote include -iquote include/Components

all: $(NAME)

$(NAME):	$(OBJ)
	$(CXX) $(CXXFLAGS) -o $(NAME) $(OBJ)

clean:
	$(RM) $(OBJ)

fclean: clean
	$(RM) $(NAME)

re : fclean all

debug: CPPFLAGS += -g3
debug: all

.PHONY: all clean fclean re debug
