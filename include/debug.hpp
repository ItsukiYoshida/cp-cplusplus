#pragma once
#include <bits/extc++.h>

const std::string RESET = "\033[0m";
const std::string RED = "\033[31m";
const std::string YELLOW = "\033[33m";

template <typename T1, typename T2>
std::ostream &operator<<(std::ostream &os, const std::pair<T1, T2> &p)
{
    os << "{" << p.first << ", " << p.second << "}";
    return os;
}

template <typename T,
          typename = std::enable_if_t<
              !std::is_same_v<T, std::string> &&
              !std::is_same_v<T, const char *> &&
              std::is_constructible_v<decltype(std::begin(std::declval<T>())), decltype(std::end(std::declval<T>()))>>>
std::ostream &operator<<(std::ostream &os, const T &v)
{
    os << "[";
    auto it = std::begin(v);
    if (it != std::end(v))
    {
        os << *it;
        for (++it; it != std::end(v); ++it)
        {
            os << ", " << *it;
        }
    }
    os << "]";
    return os;
}

template <typename K, typename V>
std::ostream &operator<<(std::ostream &os, const std::map<K, V> &m)
{
    os << "{";
    auto it = m.begin();
    if (it != m.end())
    {
        os << it->first << ": " << it->second;
        for (++it; it != m.end(); ++it)
        {
            os << ", " << it->first << ": " << it->second;
        }
    }
    os << "}";
    return os;
}

namespace DebugInternal {
    std::string trim(const std::string& s) {
        size_t first = s.find_first_not_of(" \t\n\r");
        if (std::string::npos == first) return s;
        size_t last = s.find_last_not_of(" \t\n\r");
        return s.substr(first, (last - first + 1));
    }
    std::vector<std::string> split_exprs(const std::string& s) {
        std::vector<std::string> result;
        std::string current;
        int depth = 0;
        for (char c : s) {
            if (c == ',' && depth == 0) {
                result.push_back(trim(current));
                current.clear();
            } else {
                if (c == '(' || c == '<' || c == '{') depth++;
                if (c == ')' || c == '>' || c == '}') depth--;
                current += c;
            }
        }
        result.push_back(trim(current));
        return result;
    }
    class Debugger {
        std::string prefix_str;
        std::string indent_str;
        std::vector<std::string> exprs;
        std::ostringstream oss;
        size_t index = 0;
    public:
        Debugger(const char* file, int line, const char* expr_str) : exprs(split_exprs(expr_str)) {
            std::ostringstream prefix_oss;
            prefix_oss << YELLOW << "[" << file << ":" << line << "] " << RESET;
            prefix_str = prefix_oss.str();

            std::string visible_part = std::string("[") + file + ":" + std::to_string(line) + "] ";
            indent_str.assign(visible_part.length(), ' ');
        }
        template <typename T>
        Debugger& operator,(const T& value) {
            if (index == 0) {
                oss << prefix_str;
            } else {
                oss << "\n" << indent_str;
            }
            oss << RED << exprs[index++] << RESET << ": " << value;
            return *this;
        }
        ~Debugger() {
            if (!oss.str().empty()) {
                std::cerr << "=== debug ===" << std::endl;
                std::cerr << oss.str() << std::endl;
                std::cerr << "=============" << std::endl;
            }
        }
    };
}
#define debug(...) DebugInternal::Debugger(__FILE__, __LINE__, #__VA_ARGS__), __VA_ARGS__

namespace DebugInternal
{
    void fdebug_helper(std::ostringstream&, const std::string&, bool) {}
    
    template <typename Value, typename... Tail>
    void fdebug_helper(std::ostringstream& oss, const std::string& indent, bool is_first,
                       const std::string& label, const Value& value, Tail... tail) {
        if (!is_first) {
            oss << "\n" << indent;
        }
        oss << RED << label << RESET << ": " << value;
        fdebug_helper(oss, indent, false, tail...);
    }
}

#define fdebug(...) \
    do { \
        std::ostringstream oss_body; \
        std::string indent_str; \
        std::string visible_part = std::string("[") + __FILE__ + ":" + std::to_string(__LINE__) + "] "; \
        indent_str.assign(visible_part.length(), ' '); \
        \
        DebugInternal::fdebug_helper(oss_body, indent_str, true, __VA_ARGS__); \
        \
        if (!oss_body.str().empty()) { \
            std::cerr << "=== debug ===" << std::endl; \
            std::cerr << YELLOW << "[" << __FILE__ << ":" << __LINE__ << "] " << RESET << oss_body.str() << std::endl; \
            std::cerr << "=============" << std::endl; \
        } \
    } while (0)

#define L(expr) \
    ([&]() { \
        std::ostringstream oss; \
        oss << expr; \
        return oss.str(); \
    }())
