/*
** EPITECH PROJECT, 2026
** parserhelper
** File description:
** 
*/

#ifndef PARSERHELPER_HPP
    #define PARSERHELPER_HPP
    #include <vector>
    #include <string>
    #include <iostream>
    #include <sstream>
    #include <fstream>
    #include <string.h>

class ParserHelper
{
public:
    static std::string removeFromChar(std::string line, char c)
    {
        size_t pos = line.find(c);

        if (pos == std::string::npos)
            return line;
        return line.substr(0, pos);
    }
    static std::vector<std::string>tokenizeLine(std::string line)
    {
        std::vector<std::string>vector_instruct;
        std::istringstream iss(line);
        std::string element;
        std::vector<std::string> temp;

        while (iss >> element)
            vector_instruct.push_back(element);
        return vector_instruct;
    }

    static std::vector<std::string>tokenizeByChar(const std::string &line, char delim)
    {
        std::vector<std::string>vector_instruct;
        std::stringstream ss(line);
        std::string element;

        while (getline(ss, element, delim))
            vector_instruct.push_back(element);
        return vector_instruct;
    }

    static std::vector<std::vector<std::string>>tokenizeFile(std::string path, char c)
    {
        std::ifstream file(path);
        std::string line;
        std::vector<std::vector<std::string>> vec_file;
    
        if (file.fail())
            return vec_file;
        while (getline(file, line)) {
            line = ParserHelper::removeFromChar(line, c);
            if (line.empty() || line[0] == c)
                continue;
            std::vector<std::string> vec_line = ParserHelper::tokenizeLine(line);
            if (vec_line.empty())
                continue;
            vec_file.push_back(vec_line);
        }
        return vec_file;
    }

};

#endif
