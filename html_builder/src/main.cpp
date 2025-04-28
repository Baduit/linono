#include <iostream>

#include <boost/pfr/core_name.hpp>

struct test {
	int n;
	std::string str;
};

void toto()
{
    test var{42, "Hello, World!"};

    // Outputs:
    //  n: 42
    //  str: Hello, World!
    boost::pfr::for_each_field_with_name(var,
      [](std::string_view name, const auto& value) {
        std::cout << name << ": " << value << std::endl;
    });

	std::cout << "Hello, World!" << std::endl;
}
