/*
** EPITECH PROJECT, 2026
** 
** File description:
** 
*/

#include "Circuit.hpp"
#include "Shell.hpp"

int main(int const argc, const char **argv)
{
    if (argc != 2)
        return 84;
    try {
        nts::Circuit circuit(argv);
        nts::Shell shell(circuit);
        shell.shellLoop();
        return 0;
    }
    catch (const std::exception &e) {
        std::cerr << e.what() << '\n';
        return 84;
    }   
}
