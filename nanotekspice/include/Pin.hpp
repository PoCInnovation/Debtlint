/*
** EPITECH PROJECT, 2026
** Pin
** File description:
** teckspice
*/

#ifndef PIN_HPP
    #define PIN_HPP
    #include <vector>
    #include "IComponent.hpp"

namespace nts {
    class Pin
    {
    public:
        Pin(IComponent &comp, size_t index) :
            _index(index), _component(&comp), _link(&comp, index) {}

        IComponent &getComponent()
        {
            return *_component;
        }

        std::size_t getIndex()
        {
            return _index;
        }

        void setLink(IComponent &linkComp, size_t pin)
        {
            _link = std::tuple<IComponent*, size_t>(&linkComp, pin);
        }

        nts::IComponent &getLinkComp()
        {
            return *std::get<0>(_link);
        }
        size_t getLinkIndex()
        {
            return std::get<1>(_link);
        }

    private:
        std::size_t _index; //Index dans le Comp
        IComponent *_component; //Comp dont il fait partie
        std::tuple<IComponent*, size_t> _link; // retour en arriere donc 1 seul
    
    };
}

#endif