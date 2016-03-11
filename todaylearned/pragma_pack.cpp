#include <iostream>
#include <cstddef>

#pragma pack(push, 1)
struct with_pack {
    char c;
    double d;
    int i;
};
#pragma pack(pop)

struct without_pack {
    char c;
    double d;
    int i;
};

int main()
{
    int size_with = sizeof(struct with_pack);
    int size_without = sizeof(struct without_pack);

    std::cout << "size of with_pack: " << size_with << std::endl;
    std::cout << "offsets:" << std::endl;
    std::cout << "  c: " << offsetof(struct with_pack, c) << '\n';
    std::cout << "  d: " << offsetof(struct with_pack, d) << '\n';
    std::cout << "  i: " << offsetof(struct with_pack, i) << '\n';

    std::cout << "size of without_pack: " << size_without << std::endl;
    std::cout << "offsets:" << std::endl;
    std::cout << "  c: " << offsetof(struct without_pack, c) << '\n';
    std::cout << "  d: " << offsetof(struct without_pack, d) << '\n';
    std::cout << "  i: " << offsetof(struct without_pack, i) << '\n';

    return 0;
}
