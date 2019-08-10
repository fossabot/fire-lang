#include <iostream>
#include <cstdio>
#include <cstdlib>
#include <string>

class __fire_str {
private:
    char * data;
    unsigned length;
public:
    __fire_str();
    __fire_str(const char * c);
    __fire_str(const __fire_str & s);
    ~__fire_str();

    static __fire_str __fire_from(int x);

    std::string get_string() const;
    unsigned __fire_len() const;
    int __fire_index(char c) const;
    __fire_str __fire_replace(const __fire_str & to_search, const __fire_str & replace_str);
    void __fire_upcase(unsigned first, unsigned last);
    void __fire_downcase(unsigned first, unsigned last);
    void __fire_togglecase(unsigned first, unsigned last);
    friend std::ostream & operator << (std::ostream & so, const __fire_str & s);
    friend std::istream & operator >> (std::istream & so, __fire_str & s);
    char operator[](unsigned j) const;
    char & operator[](unsigned j);
    __fire_str & operator = (const __fire_str & s);
    __fire_str & operator += (const __fire_str & s);
    friend __fire_str operator + (const __fire_str & lhs, const __fire_str & rhs);
    friend __fire_str operator + (const __fire_str & lhs, const char * rhs);
    friend __fire_str operator + (const char * lhs, const __fire_str & rhs);
    friend bool operator == (const __fire_str & lhs, const __fire_str & rhs);
    friend bool operator == (const __fire_str & lhs, const char * rhs);
    friend bool operator == (const char * lhs, const __fire_str & rhs);
    friend bool operator > (const __fire_str & lhs, const __fire_str & rhs);
    friend bool operator > (const __fire_str & lhs, const char * rhs);
    friend bool operator > (const char * lhs, const __fire_str & rhs);
    friend bool operator != (const __fire_str & lhs, const __fire_str & rhs);
    friend bool operator != (const __fire_str & lhs, const char * rhs);
    friend bool operator != (const char * lhs, const __fire_str & rhs);
    friend bool operator < (const __fire_str & lhs, const __fire_str & rhs);
    friend bool operator < (const __fire_str & lhs, const char * rhs);
    friend bool operator < (const char * lhs, const __fire_str & rhs);
    friend bool operator <= (const __fire_str & lhs, const __fire_str & rhs);
    friend bool operator <= (const __fire_str & lhs, const char * rhs);
    friend bool operator <= (const char * lhs, const __fire_str & rhs);
    friend bool operator >= (const __fire_str & lhs, const __fire_str & rhs);
    friend bool operator >= (const __fire_str & lhs, const char * rhs);
    friend bool operator >= (const char * lhs, const __fire_str & rhs);
};

__fire_str::__fire_str() {
    length = 0;
    data = new char[0];
}

char * itoa(int value, char * str, int base);
char * sprintf(char * str, char * fmt, ...);

__fire_str __fire_str::__fire_from(int x) {
    __fire_str self = __fire_str();
    self.length = (((sizeof x) * CHAR_BIT) + 2)/3 + 2;
    char s[self.length];
    sprintf(s, "%d", x);
    self.data = new char[self.length];
    strcpy(self.data, s);
    self.length = strlen(s);
    return self;
}

__fire_str::__fire_str(const char * c) {
    if (c) {
        unsigned n = 0;
        while (c[n] != '\0')
            n++;
        length = n;
        data = new char[n];
        for (unsigned j = 0; j < n; j++)
            data[j] = c[j];
    } else {
        length = 0;
        data = new char[0];
    }
}

__fire_str::__fire_str(const __fire_str & s) {
    length = s.__fire_len();
    data = new char[length];
    for (unsigned j = 0; j < length; j++)
        data[j] = s[j];
}

__fire_str::~__fire_str() {
    delete[] data;
}

unsigned __fire_str::__fire_len() const {
    return length;
}

int __fire_str::__fire_index(char c) const {
    for (unsigned j = 0; j < length; j++)
        if (data[j] == c)
            return (int) j;
    return -1;
}

std::string __fire_str::get_string() const {
    return std::string(this->data);
}

void replace(std::string & data, std::string toSearch, std::string replaceStr) {
    size_t pos = data.find(toSearch);
    while (pos != std::string::npos) {
        data.replace(pos, toSearch.size(), replaceStr);
        pos =data.find(toSearch, pos + replaceStr.size());
    }
}

__fire_str __fire_str::__fire_replace(const __fire_str & to_search, const __fire_str & replace_str) {
    std::string data = std::string(this->data);
    replace(data, to_search.get_string(), replace_str.get_string());
    __fire_str new_str = __fire_str(data.c_str());
    return new_str;
}

void __fire_str::__fire_upcase(unsigned first, unsigned last) {
    for (unsigned j = first; j < last; j++)
        if ('a' <= data[j] && data[j] <= 'z')
            data[j] -= ('a' - 'A');
}

void __fire_str::__fire_downcase(unsigned first, unsigned last) {
    for (unsigned j = first; j < last; j++)
        if ('A' <= data[j] && data[j] <= 'Z')
            data[j] += ('a' - 'A');
}

void __fire_str::__fire_togglecase(unsigned first, unsigned last) {
    for (unsigned j = first; j < last; j++)
        if ('A' <= data[j] && data[j] <= 'Z')
            data[j] += ('a' - 'A');
        else if ('a' <= data[j] && data[j] <= 'z')
        data[j] -= ('a' - 'A');
}

std::ostream & operator << (std::ostream & os, const __fire_str & s) {
    if (s.__fire_len() > 0) {
        for (unsigned j = 0; j < s.__fire_len(); j++)
            os << s[j];
    } else
        os << "";

    return os;
}

std::istream & operator >> (std::istream & is, __fire_str & s) {
    char * c = new char[1000];
    is >> c;
    s = __fire_str(c);
    delete[] c;

    return is;
}

char __fire_str::operator[](unsigned j) const {
    return data[j];
}

char & __fire_str::operator[](unsigned j) {
    return data[j];
}

__fire_str & __fire_str::operator = (const __fire_str & s) {
    if (this == & s)
        return *this;

    delete data;
    length = s.__fire_len();
    data = new char[length];
    for (unsigned j = 0; j < length; j++)
        data[j] = s[j];
    return *this;
}

__fire_str & __fire_str::operator += (const __fire_str & s) {
    unsigned __fire_len = length + s.__fire_len();
    char * str = new char[__fire_len];

    for (unsigned j = 0; j < length; j++)
        str[j] = data[j];

    for (unsigned i = 0; i < s.__fire_len(); i++)
        str[length + i] = s[i];

    delete data;
    length = __fire_len;
    data = str;
    return *this;
}

__fire_str operator + (const __fire_str & lhs, const __fire_str & rhs) {
    return __fire_str(lhs) += rhs;
}

__fire_str operator + (const __fire_str & lhs, const char * rhs) {
    return __fire_str(lhs) += __fire_str(rhs);
}

__fire_str operator + (const char * lhs, const __fire_str & rhs) {
    return __fire_str(lhs) += rhs;
}

bool operator == (const __fire_str & lhs, const __fire_str & rhs) {
    if (lhs.__fire_len() != rhs.__fire_len())
        return false;

    unsigned cap = lhs.__fire_len();
    unsigned n = 0;
    while ((n < cap) && (lhs[n] == rhs[n]))
        n++;
    return (n == cap);
}

bool operator == (const __fire_str & lhs, const char * rhs) {
    return (lhs == __fire_str(rhs));
}

bool operator == (const char * lhs, const __fire_str & rhs) {
    return (__fire_str(lhs) == rhs);
}

bool operator > (const __fire_str & lhs, const __fire_str & rhs) {
    unsigned cap = (lhs.__fire_len() < rhs.__fire_len()) ? lhs.__fire_len() : rhs.__fire_len();
    unsigned n = 0;
    while ((n < cap) && (lhs[n] == rhs[n]))
        n++;
    if (n == cap)
        return (lhs.__fire_len() > rhs.__fire_len());

    if ((('A' <= lhs[n] && lhs[n] <= 'Z') || ('a' <= lhs[n] && lhs[n] <= 'z')) && (('A' <= rhs[n] && rhs[n] <= 'Z') || ('a' <= rhs[n] && rhs[n] <= 'z'))) {
        char A = (lhs[n] & ~32);
        char B = (rhs[n] & ~32);
        if (A != B)
            return (A > B);
    }
    return lhs[n] > rhs[n];
}

bool operator > (const __fire_str & lhs, const char * rhs) {
    return (lhs > __fire_str(rhs));
}

bool operator > (const char * lhs, const __fire_str & rhs) {
    return (__fire_str(lhs) > rhs);
}

bool operator != (const __fire_str & lhs, const __fire_str & rhs) {
    return !(lhs == rhs);
}

bool operator != (const __fire_str & lhs, const char * rhs) {
    return !(lhs == rhs);
}

bool operator != (const char * lhs, const __fire_str & rhs) {
    return !(lhs == rhs);
}

bool operator < (const __fire_str & lhs, const __fire_str & rhs) {
    return !(lhs == rhs) && !(lhs > rhs);
}

bool operator < (const __fire_str & lhs, const char * rhs) {
    return !(lhs == rhs) && !(lhs > rhs);
}

bool operator < (const char * lhs, const __fire_str & rhs) {
    return !(lhs == rhs) && !(lhs > rhs);
}

bool operator <= (const __fire_str & lhs, const __fire_str & rhs) {
    return !(lhs > rhs);
}

bool operator <= (const __fire_str & lhs, const char * rhs) {
    return !(lhs > rhs);
}

bool operator <= (const char * lhs, const __fire_str & rhs) {
    return !(lhs > rhs);
}

bool operator >= (const __fire_str & lhs, const __fire_str & rhs) {
    return (lhs == rhs) || (lhs > rhs);
}

bool operator >= (const __fire_str & lhs, const char * rhs) {
    return (lhs == rhs) || (lhs > rhs);
}

bool operator >= (const char * lhs, const __fire_str & rhs) {
    return (lhs == rhs) || (lhs > rhs);
}
