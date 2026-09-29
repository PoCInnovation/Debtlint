/*
** EPITECH PROJECT, 2026
** 4040
** File description:
** 
*/

#ifndef COMPONENT4040_HPP
    #define COMPONENT4040_HPP
    #include "AComponent.hpp"
    #include <map>
    #include "Pin.hpp"
    #include "Gates.hpp"

class Component4040 : public nts::AComponent
{
public:
    Component4040() {
        _pins = {
            {9, nts::Pin(*this, 9)}, // 2^0
            {7, nts::Pin(*this, 7)}, // 2^1
            {6, nts::Pin(*this, 6)}, // 2^2
            {5, nts::Pin(*this, 5)}, // 2^3
            {3, nts::Pin(*this, 3)}, // 2^4
            {2, nts::Pin(*this, 2)}, // 2^5
            {4, nts::Pin(*this, 4)}, // 2^6
            {13, nts::Pin(*this, 13)}, // 2^7
            {12, nts::Pin(*this, 12)}, // 2^8
            {14, nts::Pin(*this, 14)}, // 2^9
            {15, nts::Pin(*this, 15)}, // 2^10
            {1, nts::Pin(*this, 1)},  // 2^11
            {10, nts::Pin(*this, 10)}, // clock
            {11, nts::Pin(*this, 11)}, // reset
            {8, nts::Pin(*this, 8)}, //Vss
            {16, nts::Pin(*this, 16)}};//Vdd

        _reset = nts::Undefined;
    }

    void simulate(std::size_t tick) final {
        (void)tick;
        _reset = nts::Undefined;
        _state = _pins.at(10).getLinkComp().getState();
        if (_state == nts::Down){
            _state = nts::Stay;
            _count += 1;
        }
        _computing.clear();
    }

    void setValue(nts::Tristate value) { (void)value; }
    nts::Tristate getValue() { return nts::Undefined; }
    nts::Tristate compute(std::size_t pin) final
    {
        if (!_computing.contains(pin)) {
            _computing[pin] = true;
            _reset = _pins.at(11).getLinkComp().compute(_pins.at(11).getLinkIndex());
        }
        nts::Tristate clock = _pins.at(10).getLinkComp().compute(_pins.at(10).getLinkIndex());

        if (_reset == nts::True){
            _count = 0;
            return nts::False;
        }
        if (clock == nts::Undefined)
            return nts::Undefined;
        if (pin != 10 && pin != 11 && pin != 8 && pin != 16){
            for (auto [ind, _] : _pins) {
                if (ind == pin){
                    return static_cast<nts::Tristate>(_count >> _power[ind] & 0b1);
                }
            }
            
        }
        return nts::Undefined;
    }
private:
    std::size_t _count = 0;
    std::map<size_t, size_t> _power = {
        {9, 0},
        {7, 1},
        {6, 2},
        {5, 3},
        {3, 4},
        {2, 5},
        {4, 6},
        {13, 7},
        {12, 8},
        {14, 9},
        {15, 10},
        {1, 11}};
    nts::Tristate _reset;

};

#endif
