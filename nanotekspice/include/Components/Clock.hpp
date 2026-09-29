/*
** EPITECH PROJECT, 2026
** Clock
** File description:
** 
*/

#ifndef CLOCK_HPP
    #define CLOCK_HPP
    #include "AComponent.hpp"
    #include <map>

class Clock : public nts::AComponent
{
public:
    Clock() {
        _pins = {{1, nts::Pin(*this, 1)}};
        _value = nts::Tristate::Undefined;
        _newValue = nts::Tristate::Undefined;
    }
    void simulate(std::size_t tick) final
    {
        (void)tick;
        updateState();
        _value = _newValue;
        if (_value == nts::Undefined)
            _newValue = nts::Undefined;
        else if (_newValue == nts::True)
            _newValue = nts::False;
        else
            _newValue = nts::True;
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
