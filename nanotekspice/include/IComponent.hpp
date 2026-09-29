/*
** EPITECH PROJECT, 2026
** Icomponent
** File description:
** 
*/

#ifndef ICOMPONENT_HPP
    #define ICOMPONENT_HPP
    #include <string>
    #include <iostream>

namespace nts
{
    enum Tristate {
        Undefined = (-true),
        True = true,
        False = false
    };

    enum State {
        Up = true,
        Down = false,
        Stay = (-true)
    };

    class IComponent
    {
        public :
            virtual ~IComponent() = default ;
            virtual void simulate(std::size_t tick) = 0;
            virtual nts::Tristate compute(std::size_t pin) = 0;
            virtual void setLink(std::size_t pin, nts::IComponent &other, std::size_t otherPin) = 0;
            virtual nts::Tristate getValue() = 0;
            virtual void setValue(nts::Tristate value) = 0;
            virtual nts::State getState() = 0;
    };
}

std::ostream &operator<<(std::ostream &stream, const nts::Tristate value);

#endif
