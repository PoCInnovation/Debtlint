/*
** EPITECH PROJECT, 2026
** 4514
** File description:
** 
*/

#ifndef COMPONENT4514_HPP
    #define COMPONENT4514_HPP
    #include "AComponent.hpp"
    #include <map>
    #include "Pin.hpp"
    #include "Gates.hpp"
    #include <array>

class Component4514 : public nts::AComponent
{
private:
    std::size_t _resultPin = 0;
    std::array<nts::Tristate, 4> _oldInputs {
        nts::Undefined,
        nts::Undefined,
        nts::Undefined,
        nts::Undefined
    };
    std::map<std::size_t, std::size_t> _outputMap {
        {0, 11},
        {1, 9},
        {2, 10},
        {3, 8},
        {4, 7},
        {5, 6},
        {6, 5},
        {7, 4},
        {8, 18},
        {9, 17},
        {10, 20},
        {11, 19},
        {12, 14},
        {13, 13},
        {14, 16},
        {15, 15}
    };
public:
    Component4514() {
        _pins = {
            {1, nts::Pin(*this, 1)}, // Strobe
            {2, nts::Pin(*this, 2)}, // input A
            {3, nts::Pin(*this, 3)}, // input B
            {21, nts::Pin(*this, 21)}, // input C
            {22, nts::Pin(*this, 22)}, // input D
            {23, nts::Pin(*this, 23)}, // Inhibit
            {11, nts::Pin(*this, 11)}, // Output 0
            {9, nts::Pin(*this, 9)}, // Output 1
            {10, nts::Pin(*this, 10)}, // Output 2
            {8, nts::Pin(*this, 8)}, // Output 3
            {7, nts::Pin(*this, 7)}, // Output 4
            {6, nts::Pin(*this, 6)}, // Output 5
            {5, nts::Pin(*this, 5)}, // Output 6
            {4, nts::Pin(*this, 4)}, // Output 7
            {18, nts::Pin(*this, 18)}, // Output 8
            {17, nts::Pin(*this, 17)}, // Output 9
            {20, nts::Pin(*this, 20)}, // Output 10
            {19, nts::Pin(*this, 19)}, // Output 11
            {14, nts::Pin(*this, 14)}, // Output 12
            {13, nts::Pin(*this, 13)}, // Output 13
            {16, nts::Pin(*this, 16)}, // Output 14
            {15, nts::Pin(*this, 15)}, // Output 15
            {12, nts::Pin(*this, 12)},// Vss
            {24, nts::Pin(*this, 24)}};// Vdd
    }

    void simulate(std::size_t tick) final {
        (void)tick;
    }

    void setValue(nts::Tristate value) { (void)value; }
    nts::Tristate getValue() { return nts::Undefined;}

    void computeResultPin(nts::Tristate pinA, nts::Tristate pinB, 
        nts::Tristate pinC, nts::Tristate pinD)
    {
        std::size_t valA = (pinA == nts::True) ? 1 : 0;
        std::size_t valB = ((pinB == nts::True) ? 1 : 0) << 1;
        std::size_t valC = ((pinC == nts::True) ? 1 : 0) << 2;
        std::size_t valD = ((pinD == nts::True) ? 1 : 0) << 3;

        _oldInputs = {pinA, pinB, pinC, pinD};
        _resultPin = (0b1 & valA) + (0b10 & valB) + (0b100 & valC) + (0b1000 & valD);
    }

    bool isOutput(std::size_t pin)
    {
        for (const auto&[_, output] : _outputMap) {
            if (output == pin) {
                return true;
            }
        }
        return false;
    }

    nts::Tristate compute(std::size_t pin) final
    {
        if (!isOutput(pin))
            return nts::Undefined;
        if (computePin(23) == nts::True)
            return nts::False;
        if (computePin(1) == nts::True){
            computeResultPin(computePin(2), computePin(3), computePin(21), computePin(22));
        } else {
            computeResultPin(_oldInputs[0], _oldInputs[1], _oldInputs[2], _oldInputs[3]);
        }
        if (pin == _outputMap.at(_resultPin)){
            return nts::True;
        } else {
            return nts::False;
        }
    }
};

#endif
