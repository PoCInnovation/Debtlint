/*
** EPITECH PROJECT, 2026
** 4512
** File description:
** 
*/

#ifndef COMPONENT4512_HPP
    #define COMPONENT4512_HPP
    #include "AComponent.hpp"
    #include <map>
    #include "Pin.hpp"
    #include "Gates.hpp"

class Component4512 : public nts::AComponent
{
public:
    Component4512() {
        _pins = {
            {1, nts::Pin(*this, 1)}, // X0 data
            {2, nts::Pin(*this, 2)}, // x1 data
            {3, nts::Pin(*this, 3)}, // x2 data
            {4, nts::Pin(*this, 4)}, // x3 data
            {5, nts::Pin(*this, 5)}, // x4 data
            {6, nts::Pin(*this, 6)}, // x5 data
            {7, nts::Pin(*this, 7)}, // x6 data
            {9, nts::Pin(*this, 9)}, // X7 data
            {10, nts::Pin(*this, 10)}, // inhibit
            {15, nts::Pin(*this, 15)}, // OE
            {11, nts::Pin(*this, 11)}, // Address Input A
            {12, nts::Pin(*this, 12)}, // Address Input B
            {13, nts::Pin(*this, 13)}, // Address Input C
            {14, nts::Pin(*this, 14)}, // Z Output
            {8, nts::Pin(*this, 8)}, //Vss
            {16, nts::Pin(*this, 16)}};//Vdd
    }
    void simulate(std::size_t tick) final {(void)tick;}
    void setValue(nts::Tristate value) { (void)value; }
    nts::Tristate getValue() { return nts::Undefined;}

    void computeResultPin(nts::Tristate pinA, nts::Tristate pinB, nts::Tristate pinC)
    {
        std::size_t valA = (pinA == nts::True) ? 1 : 0;
        std::size_t valB = ((pinB == nts::True) ? 1 : 0) << 1;
        std::size_t valC = ((pinC == nts::True) ? 1 : 0) << 2;

        _resultPin = (0b1 & valA) + (0b10 & valB) + (0b100 & valC);
        _resultPin += (_resultPin == 7) ? 2 : 1;
    }

    nts::Tristate compute(std::size_t pin) final
    {
        if (pin == 14 && computePin(15) != nts::True) {
            if (computePin(10) == nts::True) {
                return nts::False;
            }
            computeResultPin(computePin(11), computePin(12), computePin(13));
            return computePin(_resultPin);
        }
        return nts::Undefined;
    }
private:
    std::size_t _resultPin = 0;
};

#endif
