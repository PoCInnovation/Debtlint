/*
** EPITECH PROJECT, 2026
** Acomponent
** File description:
** teckspice
*/

#ifndef ACOMP_HPP
    #define ACOMP_HPP
    #include "IComponent.hpp"
    #include "Pin.hpp"
    #include "Gates.hpp"
    #include <map>

namespace nts {
    class AComponent: public IComponent
    {
    public:
        nts::Tristate computePin(int pin)
        {
            return _pins.at(pin).getLinkComp().compute(_pins.at(pin).getLinkIndex());
        }

        void setLink(std::size_t pin, nts::IComponent &other, std::size_t otherPin) final
        {
            _pins.at(pin).setLink(other, otherPin);
        }

        void updateState()
        {
            if (_value != nts::True && _newValue == nts::True){
                _state = nts::Up;
            } else if (_value != nts::False && _newValue == nts::False) {
                _state = nts::Down;
            } else {
                _state = nts::Stay;
            }
        }

        nts::State getState() final { return _state; }
    protected:
        nts::Tristate _value;
        nts::Tristate _newValue;
        std::map<size_t, nts::Pin> _pins;
        nts::State _state;
        std::map<size_t, nts::Tristate> _computed;
        std::map<size_t, bool> _computing;
        size_t _saveTick;
    };
}

#endif