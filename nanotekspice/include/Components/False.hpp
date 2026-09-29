/*
** EPITECH PROJECT, 2026
** false
** File description:
** 
*/

#ifndef FALSE_HPP
    #define FALSE_HPP
    #include "AComponent.hpp"
    #include <map>

class False : public nts::AComponent
{
public:
    False() {
        _pins = {{1, nts::Pin(*this, 1)}};
        _value = nts::Tristate::False;
        _newValue = nts::Tristate::False;
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

//Pin False:
//size_t _Index; 1
//nts::Tristate _value; // U
//nts::Tristate _NextValue; // 1
//IComponent& _Component; *this
//std::tuple<IComponent&, size_t>_Link; ()

#endif
