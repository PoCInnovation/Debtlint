/*
** EPITECH PROJECT, 2026
** factory
** File description:
** 
*/

#ifndef COMPONENTFACTORY_HPP
    #define COMPONENTFACTORY_HPP
    #include <map>
    #include <memory>
    #include <iostream>
    #include "IComponent.hpp"
    #include "4008.hpp"
    #include "4040.hpp"
    #include "4512.hpp"
    #include "4514.hpp"
    #include "4001.hpp"
    #include "4011.hpp"
    #include "4030.hpp"
    #include "4069.hpp"
    #include "4071.hpp"
    #include "4081.hpp"
    #include "And.hpp"
    #include "Clock.hpp"
    #include "False.hpp"
    #include "Input.hpp"
    #include "Not.hpp"
    #include "Or.hpp"
    #include "Output.hpp"
    #include "True.hpp"
    #include "Xor.hpp"
    #include "Nand.hpp"
    #include "Nor.hpp"

class ComponentFactory
{
public:
    ComponentFactory()
    {
        _factory["4512"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Component4512>(); };
        _factory["4514"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Component4514>(); };
        _factory["4040"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Component4040>(); };
        _factory["4008"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Component4008>(); };
        _factory["4001"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Component4001>(); };
        _factory["4011"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Component4011>(); };
        _factory["4030"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Component4030>(); };
        _factory["4069"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Component4069>(); };
        _factory["4071"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Component4071>(); };
        _factory["4081"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Component4081>(); };
        _factory["and"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<And>(); };
        _factory["clock"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Clock>(); };
        _factory["false"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<False>(); };
        _factory["input"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Input>(); };
        _factory["not"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Not>(); };
        _factory["or"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Or>(); };
        _factory["output"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Output>(); };
        _factory["true"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<True>(); };
        _factory["xor"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Xor>(); };
        _factory["nand"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Nand>(); };
        _factory["nor"] = []() -> std::unique_ptr<nts::IComponent> { return std::make_unique<Nor>(); };
    }

    std::unique_ptr<nts::IComponent> createComponent(const std::string &type)
    {
        auto it = _factory.find(type);
    
        if (it != _factory.end())
            return it->second();
        return nullptr;
    }

private:
    std::map<std::string, std::unique_ptr<nts::IComponent>(*)(void)> _factory;
};
#endif
