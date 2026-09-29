/*
** EPITECH PROJECT, 2026
** true
** File description:
** 
*/

#ifndef TRUE_HPP
    #define TRUE_HPP
    #include "AComponent.hpp"
    #include <map>

class True : public nts::AComponent
{
public:
    True() {
        _pins = {{1, nts::Pin(*this, 1)}};
        _value = nts::Tristate::True;
        _newValue = nts::Tristate::True;
    }
    void simulate(std::size_t tick) final
    {
        (void)tick;
        _value = _newValue;
    }

    nts::Tristate compute(std::size_t pin) final
    {
        (void)pin;
        return _value;
    }

    nts::Tristate getValue()
    {
        return _value;
    }
    void setValue(nts::Tristate value)
    {
        _newValue = value;
    }
};

#endif
