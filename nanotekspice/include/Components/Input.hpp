/*
** EPITECH PROJECT, 2026
** Input
** File description:
** 
*/

#ifndef INPUT_HPP
    #define INPUT_HPP
    #include "AComponent.hpp"
    #include <map>

class Input : public nts::AComponent
{
public:
    Input() {
        _pins = {{1, nts::Pin(*this, 1)}};
        _value = nts::Tristate::Undefined;
        _newValue = nts::Tristate::Undefined;
    }

    void simulate(std::size_t tick) final
    {
        (void)tick;
        updateState();
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

//Pin Input:
//size_t _Index; 1
//nts::Tristate _value; // U
//nts::Tristate _NextValue; // 1
//IComponent& _Component; *this
//std::tuple<IComponent&, size_t>_Link; ()

#endif
